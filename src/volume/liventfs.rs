use crate::devio::volume::get_ntfs_volume_data;
use crate::errors::WinThingError;
use crate::file::helper::query_file_record;
use crate::mft::MftOutputBuffer;
use crate::volume::mapping::FolderMapping;
use mft::MftEntry;
use std::fs::File;
use std::os::windows::io::AsRawHandle;
use winapi::um::winioctl::NTFS_VOLUME_DATA_BUFFER;
use winstructs::ntfs::mft_reference::MftReference;

/// Struct for interacting with a live NTFS volume via Windows API
///
pub struct WindowsLiveNtfs {
    volume_path: String,
    volume_handle: File,
    ntfs_volume_data: NTFS_VOLUME_DATA_BUFFER,
}
impl WindowsLiveNtfs {
    /// Create a WindowsLiveNtfs from the volume path
    ///
    pub fn from_volume_path(volume_path: &str) -> Result<Self, WinThingError> {
        let file_handle = File::open(&volume_path)?;

        let ntfs_volume_data = get_ntfs_volume_data(
            file_handle.as_raw_handle() as _
        )?;

        Ok(WindowsLiveNtfs {
            volume_path: volume_path.to_string(),
            volume_handle: file_handle,
            ntfs_volume_data: ntfs_volume_data,
        })
    }

    /// Get the handle reference to this volume
    ///
    pub fn get_handle(&self) -> &File {
        &self.volume_handle
    }

    /// Get an MftEntry for a given entry number
    ///
    pub fn get_mft_entry(&mut self, entry: i64) -> Result<MftEntry, WinThingError> {
        let raw_buffer = query_file_record(
            self.volume_handle.as_raw_handle() as _,
            entry,
            self.ntfs_volume_data.BytesPerFileRecordSegment,
        )?;

        let mft_buffer = MftOutputBuffer::from_buffer(&raw_buffer[..])?;

        mft_buffer.as_entry()
    }

    /// Create folder mapping
    ///
    pub fn get_folder_mapping(&self) -> FolderMapping {
        // Create the folder mapping
        let mut folder_mapping = FolderMapping::new();

        // Iterate over live MFT entries
        let entry_iter = self.get_entry_iterator();
        for entry_result in entry_iter {
            match entry_result {
                Ok(entry) => {
                    // We only want directories
                    if !entry.is_dir() {
                        continue;
                    }

                    let mut l_entry = entry.header.record_number;
                    let mut l_sequence = entry.header.sequence;

                    // if entry is child, set entry and sequence to parent
                    if entry.header.base_reference.entry != 0 {
                        l_entry = entry.header.base_reference.entry;
                        l_sequence = entry.header.base_reference.sequence;
                    }

                    // Get the best name attribute or <NA>
                    let fn_attr = match entry.find_best_name_attribute() {
                        Some(fn_attr) => fn_attr,
                        None => continue,
                    };

                    // Entry reference for our key
                    let entry_reference = MftReference::new(l_entry, l_sequence);

                    // Add this entry to the folder mapping
                    folder_mapping.add_mapping(entry_reference, fn_attr.name, fn_attr.parent);
                }
                Err(error) => {
                    eprintln!("{:?}", error);
                }
            }
        }

        folder_mapping
    }

    /// Calculate the highest possible mft entry number
    ///
    fn get_max_entry(&self) -> u64 {
        (unsafe { *self.ntfs_volume_data.MftValidDataLength.QuadPart() } as u64
            / self.ntfs_volume_data.BytesPerFileRecordSegment as u64)
    }

    /// Get entry iterator
    ///
    pub fn get_entry_iterator(&self) -> LiveMftEntryIterator {
        let last_entry = self.get_max_entry();

        LiveMftEntryIterator {
            live_ntfs: self.clone(),
            current_entry: last_entry as i64 - 1,
        }
    }
}
impl Clone for WindowsLiveNtfs {
    fn clone(&self) -> Self {
        Self::from_volume_path(&self.volume_path).expect(&format!(
            "Unable to create WindowsLiveNtfs from {}",
            self.volume_path
        ))
    }
}

/// Iterator to iterate mft entries on a live NTFS volume. The iterator
/// returns entries in reverse order (highest to lowest) which maximizes
/// performance due to Windows API because FSCTL_GET_NTFS_FILE_RECORD
/// retrieves the first file record that is in use and is of a lesser than or equal
/// ordinal value to the requested file reference number.
/// The current entry must start at the highest to lowest and be one less than
/// the max entry
///
pub struct LiveMftEntryIterator {
    live_ntfs: WindowsLiveNtfs,
    current_entry: i64,
}
impl Iterator for LiveMftEntryIterator {
    type Item = Result<MftEntry, WinThingError>;

    // It is fastest to iterate file entries from highest to lowest becuase
    // the Windows API fetches the lowest allocated entry if an entry is queried
    // that is unallocated. This prevents us from having to iterate through blocks
    // of unallocated entries (in which case the same entry will be returned till the
    // next allocated) until we find the next allocated.
    fn next(&mut self) -> Option<Result<MftEntry, WinThingError>> {
        while self.current_entry >= 0 {
            // Get MFT entry for current entry
            let mft_entry = match self.live_ntfs.get_mft_entry(self.current_entry as i64) {
                Ok(entry) => entry,
                Err(error) => {
                    self.current_entry -= 1;
                    return Some(Err(error));
                }
            };

            // Deincrement the entry by 1
            self.current_entry = mft_entry.header.record_number as i64 - 1;

            return Some(Ok(mft_entry));
        }

        None
    }
}

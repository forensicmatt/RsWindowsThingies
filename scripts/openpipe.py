import sys
import time
import json
import win32pipe, win32file, pywintypes


def read_pipe(pipe_name):
    handle = win32pipe.CreateNamedPipe(
        pipe_name,
        win32pipe.PIPE_ACCESS_INBOUND,
        win32pipe.PIPE_TYPE_MESSAGE | win32pipe.PIPE_READMODE_MESSAGE | win32pipe.PIPE_WAIT,
        1,
        65536, 
        65536,
        0,
        None
    )

    win32pipe.ConnectNamedPipe(
        handle, 
        None
    )
    print("got client")

    while True:
        status, message = win32file.ReadFile(handle, 64*1024)
        json_str = message.decode("utf-8", errors='replace')
        print(json_str)


def main():
    PIPE_NAME = sys.argv[1]
    read_pipe(PIPE_NAME)


if __name__ == "__main__":
    main()

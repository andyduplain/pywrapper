# Python Wrapper

This is a simple wrapper to make running python scripts as easy as if they were normal executables. This is really just an issue on Windows, however using this wrapper on all platforms will make it easier to maintain, especially if we need to perform additional steps like setting-up environment variables, etc.

- The executables are installed into the `bin` directory with the same names as the scripts, except for the file extension.
- The `bin` directory should be in your path.
- The scripts are installed in the sibling `scripts` directory.
- The list of executable file names is kept in the `install_names.txt` file.

## Windows Setup

```
.\setup.ps1
```

## Linux/macOS Setup

```
./setup.sh
```

## Windows Testing

```
PS C:\> cd \work\pywrapper
PS C:\work\pywrapper> .\setup.ps1
   Compiling anyhow v1.0.66
   Compiling pywrapper v0.1.0 (C:\work\pywrapper)
    Finished release [optimized] target(s) in 1.57s
   Installing bin\chbranch.exe
   Installing bin\chclone.exe
PS C:\work\pywrapper> $env:path += ";$pwd\bin"
PS C:\work\pywrapper> pushd \
PS C:\> chbranch hello world
I am \\?\C:\work\pywrapper\scripts\chbranch.py called with arguments: ['hello', 'world']
PS C:\> popd
```

## Linux/macOS Testing

```
$ cd source/pywrapper
$ source/pywrapper> ./setup.sh
   Compiling anyhow v1.0.66
   Compiling pywrapper v0.1.0 (/home/andy/source/pywrapper)
    Finished release [optimized] target(s) in 1.57s
   Installing bin/chbranch
   Installing bin/chclone
$ export PATH=$PATH:$(pwd)/bin
$ (cd /; chbranch hello world)
I am /home/andy/pywrapper/scripts/chbranch.py called with arguments: ['hello', 'world']
```

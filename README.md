# VNU-LIC PDF Unlocker

A reverse-engineered tool to unlock encrypted PDFs from the VNU-LIC.

## Overview

PDFs that are downloaded from VNU-LIC actually do not have password as your PDF viewer tell you. 
It can only be opened by the customized DLL with proper prefix to the PDF path.

This project generates a decryption prefix based on your machine's hardware UUID and uses the customized MuPDF DLL to remove encryption.

## Requirements

- Windows (32-bit MSVC target)
- `mupdf-exp-dll-x86.dll` in the same directory
- Rust toolchain

## Usage

**Option 1: Download prebuilt executable**

Download the latest release from the [Releases](https://github.com/gawgua/lic-unlock/releases) page, then run:

```bash
.\lic-unlock.exe <path-to-locked-pdf>
```

Or drag & drop the PDF onto the .exe (as the video demo below).

**Option 2: Build from source**

```bash
git clone https://github.com/gawgua/lic-unlock.git
cd lic-unlock
cargo run -- <path-to-locked-pdf>
```

The decrypted file will be saved as `decrypted-<original-filename>.pdf` in the current directory.

## Demo
https://github.com/user-attachments/assets/c67b5cdc-80a0-4885-8672-48a7bfdee93b

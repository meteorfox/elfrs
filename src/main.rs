use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

/// The ELF magic number.
pub const ELFMAG: &[u8; 4] = b"\x7FELF";

/// Sizeof ELF magic number.
pub const SELFMAG: usize = 4;

/// Byte index identifies the architecture for this binary.
pub const EI_CLASS: usize = 4;

/// Class is invalid.
pub const ELFCLASSNONE: u8 = 0;

/// This defines the 32-bit architecture.
pub const ELFCLASS32: u8 = 1;

/// This defines the 64-bit architecture.
pub const ELFCLASS64: u8 = 2;

/// Sizeof ELF e_ident array.
pub const EI_NIDENT: usize = 16;

/// Byte index specifies the data encoding of the processor-specific data in the file.
pub const EI_DATA: usize = 5;

/// Unknown data format.
pub const ELFDATANONE: u8 = 0;

/// Two's complement, little-endian.
pub const ELFDATA2LSB: u8 = 1;

/// Two's complement, big-endian.
pub const ELFDATA2MSB: u8 = 2;

/// Byte index specifiies the version number of the ELF specification
pub const EI_VERSION: usize = 6;

/// Invalid version.
pub const EV_NONE: u8 = 0;

/// Current version (1).
pub const EV_CURRENT: u8 = 1;

/// Byte index identifies the operating system and ABI to which the object is targeted.  
/// Some fields in other ELF structures have flags and values that have
/// platform-specific meanings; the interpretation of those fields is determined by the value of this byte.
pub const EI_OSABI: usize = 7;

/// Same as ELFOSABI_SYSV
pub const ELFOSABI_NONE: u8 = 0;

/// UNIX System V ABI.
pub const ELFOSABI_SYSV: u8 = 0;

/// HP-UX ABI
pub const ELFOSABI_HPUX: u8 = 1;

/// NetBSD ABI
pub const ELFOSABI_NETBSD: u8 = 2;

/// Linux ABI. Same as ELFOSABI_GNU.
pub const ELFOSABI_LINUX: u8 = 3;

/// Solaris ABI
pub const ELFOSABI_SOLARIS: u8 = 6;

/// AIX ABI
pub const ELFOSABI_AIX: u8 = 7;

/// IRIX ABI
pub const ELFOSABI_IRIX: u8 = 8;

/// FreeBSD ABI
pub const ELFOSABI_FREEBSD: u8 = 9;

/// Compaq TRU64 UNIX ABI
pub const ELFOSABI_TRU64: u8 = 10;

/// Novell Modesto ABI
pub const ELFOSABI_MODESTO: u8 = 11;

/// OpenBSD ABI
pub const ELFOSABI_OPENBSD: u8 = 12;

/// ARM EABI
pub const ELFOSABI_ARM_AEABI: u8 = 64;

/// ARM architecture ABI
pub const ELFOSABI_ARM: u8 = 97;

/// Stand-alone (embedded) ABI
pub const ELFOSABI_STANDALONE: u8 = 255;

/// Byte index specifices the version of the ABI to which the object is targeted.
/// This field is used to distinguish among incompatible versions of  an  ABI.
/// The  interpretation  of  this version number is dependent on the ABI identified by the EI_OSABI field.
/// Applications conforming to this specification use the value 0.
pub const EI_ABIVERSION: usize = 8;

#[derive(StructOpt, Debug)]
#[structopt(name = "elfrs", about = "A simple ELF file parser tool.")]
struct Opt {
    /// Input ELF file
    #[structopt(parse(from_os_str))]
    input_elf: PathBuf,

    /// Output ELF file
    #[structopt(parse(from_os_str))]
    output_elf: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    println!("Parsing ELF file [{:?}]...", opt.input_elf);

    // Parse an ELF file
    let mut fd = File::open(opt.input_elf).expect("could not open file");

    // Read ELF magic and full ident array
    let mut ident = [0_u8; EI_NIDENT];
    let n = fd.read(&mut ident[..]).expect("something happened");

    // Abort quickly if could not read e_ident or magic number is not valid
    if n < EI_NIDENT {
        panic!("Unexpected or malform ELF file.");
    } else if n < SELFMAG {
        panic!("Failed to read file's magic number.");
    } else if &ident[..SELFMAG] != ELFMAG {
        eprintln!("{:02X?}", &ident[..SELFMAG]);
        panic!("Unknown or bad magic number.");
    }

    // Parse ELF class
    let class = ident[EI_CLASS];
    match class {
        ELFCLASS64 => {
            println!("ELF64");
        }
        ELFCLASS32 => {
            println!("ELF32");
        }
        _ => {
            panic!(format!("Invalid ELF class {:x}", class));
        }
    }

    // Parsa Data encoding
    let endianess = ident[EI_DATA];
    match endianess {
        ELFDATA2LSB => {
            println!("2's complement, little-endian");
        }
        ELFDATA2MSB => {
            println!("2's complement, big-endian");
        }
        _ => {
            panic!(format!("Unknown ELF DATA format {:x}", endianess));
        }
    }

    // Parse ELF Version
    let version = ident[EI_VERSION];
    match version {
        EV_CURRENT => {
            println!("1 (current)");
        }
        _ => {
            panic!(format!("Unknown ELF Version `{:x}`", version));
        }
    }

    // Parse target OS / ABI
    let osabi = ident[EI_OSABI];
    match osabi {
        ELFOSABI_SYSV => {
            println!("UNIX - System V");
        }
        ELFOSABI_LINUX => {
            println!("Linux");
        }
        _ => {
            panic!(format!("OS/ABI Version `{:x}` not supported", osabi));
        }
    }

    // Parse ABI Version
    let abi = ident[EI_ABIVERSION];
    if abi != 0 {
        panic!(format!("Extended ABI version `{:x}` not supported", abi));
    }
    println!("{:x}", abi);
}

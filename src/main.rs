use std::env::args;
use std::ffi::CStr;
use std::path::Path;

use nix::libc;
use nix::sys::{stat, stat::Mode, stat::SFlag};

use chrono::{DateTime, NaiveDateTime, Utc};

fn main() {
    let prog_name = args().nth(0).unwrap();
    if args().len() == 1 {
        eprintln!(
            "{}: missing operand\nTry '{} --help' for more information.",
            prog_name, prog_name
        )
    } else {
        args().skip(1).for_each(|file_name| {
            let p = Path::new(&file_name);
            let s = stat::stat(p).unwrap();

            println!("  File: {}", file_name);
            println!(
                "  Size: {}\t\tBlocks: {}\tIO Blocks: {}\t{}",
                s.st_size,
                s.st_blocks,
                s.st_blksize,
                file_type(s.st_mode, false)
            );
            println!(
                "Device: {:x}h/{}d\tInode: {}\tLinks: {}",
                s.st_dev, s.st_dev, s.st_ino, s.st_nlink
            );
            println!(
                "Access: (0{:o}/{})\tUid: (\t{}/\t{})\tGid: (\t{}/\t{})",
                s.st_mode & 0x8fff,
                file_mode_string(s.st_mode),
                s.st_uid,
                user_name(s.st_uid),
                s.st_gid,
                group_name(s.st_gid),
            );

            println!(
                "Access: {}",
                timestamp_to_str(s.st_atime, s.st_atime_nsec as u32)
            );
            println!(
                "Modify: {}",
                timestamp_to_str(s.st_mtime, s.st_mtime_nsec as u32)
            );
            println!(
                "Change: {}",
                timestamp_to_str(s.st_ctime, s.st_ctime_nsec as u32)
            );
            println!(" Birth: -");
        })
    }
}

fn file_type(mode: u32, short: bool) -> String {
    let result = mode & SFlag::S_IFMT.bits();
    let mode = SFlag::from_bits(result).unwrap_or(SFlag::S_IFMT);
    if short {
        match mode {
            SFlag::S_IFBLK => "b".to_string(),
            SFlag::S_IFCHR => "c".to_string(),
            SFlag::S_IFDIR => "d".to_string(),
            SFlag::S_IFIFO => "p".to_string(),
            SFlag::S_IFLNK => "l".to_string(),
            SFlag::S_IFREG => "-".to_string(),
            SFlag::S_IFSOCK => "s".to_string(),
            _ => "?".to_string(),
        }
    } else {
        match mode {
            SFlag::S_IFBLK => "block device".to_string(),
            SFlag::S_IFCHR => "character device".to_string(),
            SFlag::S_IFDIR => "directory".to_string(),
            SFlag::S_IFIFO => "FIFO/pipe".to_string(),
            SFlag::S_IFLNK => "symlink".to_string(),
            SFlag::S_IFREG => "regular file".to_string(),
            SFlag::S_IFSOCK => "socket".to_string(),
            _ => "unknown?".to_string(),
        }
    }
}

fn file_mode_string(mode: u32) -> String {
    let mut s = String::new();
    s.push_str(&file_type(mode, true));
    s.push_str(bits_cmp(mode, Mode::S_IRUSR, "r", "-"));
    s.push_str(bits_cmp(mode, Mode::S_IWUSR, "w", "-"));
    s.push_str(bits_cmp(
        mode,
        Mode::S_ISUID,
        bits_cmp(mode, Mode::S_IXUSR, "s", "S"),
        bits_cmp(mode, Mode::S_IXUSR, "x", "-"),
    ));

    s.push_str(bits_cmp(mode, Mode::S_IRGRP, "r", "-"));
    s.push_str(bits_cmp(mode, Mode::S_IWGRP, "w", "-"));
    s.push_str(bits_cmp(
        mode,
        Mode::S_ISGID,
        bits_cmp(mode, Mode::S_IXGRP, "s", "S"),
        bits_cmp(mode, Mode::S_IXGRP, "x", "-"),
    ));

    s.push_str(bits_cmp(mode, Mode::S_IROTH, "r", "-"));
    s.push_str(bits_cmp(mode, Mode::S_IWOTH, "w", "-"));
    s.push_str(bits_cmp(
        mode,
        Mode::S_ISVTX,
        bits_cmp(mode, Mode::S_IXOTH, "t", "T"),
        bits_cmp(mode, Mode::S_IXOTH, "x", "-"),
    ));

    s
}

fn bits_cmp(mode: u32, target: Mode, s1: &'static str, s2: &'static str) -> &'static str {
    if mode & target.bits() != 0 {
        s1
    } else {
        s2
    }
}

fn user_name(uid: u32) -> &'static str {
    unsafe {
        let pw = libc::getpwuid(uid).as_ref().unwrap();
        let name = CStr::from_ptr(pw.pw_name).to_str().unwrap();
        return name;
    }
}

fn group_name(gid: u32) -> &'static str {
    unsafe {
        let gr = libc::getgrgid(gid).as_ref().unwrap();
        let name = CStr::from_ptr(gr.gr_name).to_str().unwrap();
        return name;
    }
}

fn timestamp_to_str(secs: i64, nsecs: u32) -> String {
    let nt = NaiveDateTime::from_timestamp(secs, nsecs);
    let date_time = DateTime::<Utc>::from_utc(nt, Utc);
    date_time.to_rfc3339()
}

use std::fs;
fn read_mmap(name: &str) -> Vec<u32> {
    let buf = fs::read_to_string(name).expect("Cant read input");
    buf.trim_end()
        .replace("\n", "")
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect()
}

#[derive(Clone, Debug, Copy)]
enum Block {
    File(u32, u32),
    Free(u32),
}

fn main() {
    let mmap = read_mmap("input9.txt");

    let mut disk = Vec::new();
    let mut file = true;
    let mut id = 0;
    for digit in &mmap {
        if file {
            disk.extend(std::iter::repeat(Some(id)).take((*digit).try_into().unwrap()));
            id += 1;
        } else {
            disk.extend(std::iter::repeat(None).take((*digit).try_into().unwrap()));
        }

        file = !file;
    }

    loop {
        let last_digit = disk.len() - disk.iter().rev().position(|x| x.is_some()).unwrap() - 1;
        let first_free = disk.iter().position(|x| x.is_none()).unwrap();
        if first_free > last_digit {
            break;
        }
        disk[first_free] = disk[last_digit];
        disk[last_digit] = None;
    }

    let mut checksum = 0;
    for (i, digit) in disk.iter().enumerate() {
        if let Some(digit) = digit {
            checksum += u64::try_from(i).unwrap() * digit;
        }
    }
    println!("{checksum}");

    let mut disk = Vec::new();
    let mut file = true;
    let mut id = 0;
    for digit in mmap {
        if file {
            disk.push(Block::File(digit, id));
            id += 1;
        } else {
            disk.push(Block::Free(digit));
        }
        file = !file;
    }

    for id in (0..id).into_iter().rev() {
        let file_index = disk
            .iter()
            .position(|&x| match x {
                Block::File(_, i) => i == id,
                Block::Free(_) => false,
            })
            .unwrap();
        let size = match disk[file_index] {
            Block::File(size, _) => size,
            Block::Free(_) => panic!(),
        };
        let free = disk.iter().position(|&x| match x {
            Block::File(_, _) => false,
            Block::Free(s) => s >= size,
        });

        if let Some(free_index) = free {
            if free_index > file_index {
                continue;
            }
            disk[free_index] = match disk[free_index] {
                Block::File(_, _) => panic!(),
                Block::Free(s) => Block::Free(s - size),
            };
            let file = disk[file_index];
            disk[file_index] = Block::Free(size);
            disk.insert(free_index, file);
        }
    }
    let mut disk_vec = Vec::new();
    for block in disk {
        match block {
            Block::File(size, id) => {
                disk_vec.extend(std::iter::repeat(Some(id)).take((size).try_into().unwrap()))
            }
            Block::Free(size) => {
                disk_vec.extend(std::iter::repeat(None).take((size).try_into().unwrap()))
            }
        }
    }

    let mut checksum = 0;
    for (i, digit) in disk_vec.iter().enumerate() {
        if let Some(digit) = digit {
            checksum += u64::try_from(i).unwrap() * u64::from(*digit);
        }
    }
    println!("{checksum}");
}

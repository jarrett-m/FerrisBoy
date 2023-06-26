pub struct Cart {
    filename: String,
    rom_size: usize,
    rom_data: Vec<u8>,
    rom_header: RomHeader,
}

static LIC_CODE: [&str; 0xA5] = [
    "None",
    "Nintendo R&D1",
    "",
    "",
    "",
    "",
    "",
    "",
    "Capcom",
    "",
    "",
    "",
    "",
    "",
    "",
    "Electronic Arts",
    "",
    "",
    "",
    "Hudson Soft",
    "b-ai",
    "",
    "kss",
    "",
    "pow",
    "",
    "",
    "PCM Complete",
    "san-x",
    "",
    "",
    "Kemco Japan",
    "seta",
    "",
    "",
    "",
    "",
    "",
    "Viacom",
    "Nintendo",
    "Bandai",
    "Ocean/Acclaim",
    "Konami",
    "Hector",
    "",
    "Taito",
    "Hudson",
    "Banpresto",
    "",
    "Ubi Soft",
    "Atlus",
    "",
    "Malibu",
    "",
    "angel",
    "Bullet-Proof",
    "",
    "irem",
    "",
    "Absolute",
    "Acclaim",
    "Activision",
    "American sammy",
    "Konami",
    "Hi tech entertainment",
    "LJN",
    "Matchbox",
    "Mattel",
    "Milton Bradley",
    "Titus",
    "Virgin",
    "",
    "",
    "LucasArts",
    "",
    "Ocean",
    "",
    "Electronic Arts",
    "",
    "Infogrames",
    "Interplay",
    "Broderbund",
    "sculptured",
    "",
    "sci",
    "",
    "",
    "THQ",
    "Accolade",
    "",
    "",
    "",
    "",
    "misawa",
    "",
    "",
    "lozc",
    "",
    "",
    "Tokuma Shoten Intermedia",
    "Tsukuda Original",
    "",
    "",
    "Chunsoft",
    "Video system",
    "Ocean/Acclaim",
    "",
    "Varie",
    "Yonezawa/s'pal",
    "Kaneko",
    "",
    "Pack in soft",
    "",
    "",
    "",
    "",
    "Konami (Yu-Gi-Oh!)",
    "Malibu",
    "",
    "angel",
    "Bullet-Proof",
    "",
    "irem",
    "",
    "Absolute",
    "Acclaim",
    "Activision",
    "American sammy",
    "Konami",
    "Malibu",
    "",
    "angel",
    "Bullet-Proof",
    "",
    "irem",
    "",
    "Absolute",
    "Acclaim",
    "Activision",
    "American sammy",
    "Konami",
    "Malibu",
    "",
    "angel",
    "Bullet-Proof",
    "",
    "irem",
    "",
    "Absolute",
    "Acclaim",
    "Activision",
    "American sammy",
    "Konami",
    "Malibu",
    "",
    "angel",
    "Bullet-Proof",
    "",
    "irem",
    "",
    "Absolute",
    "Acclaim",
    "Activision",
    "American sammy",
    "Konami",
];

static ROM_TYPES: [&str; 35] = [
    "ROM ONLY",
    "MBC1",
    "MBC1+RAM",
    "MBC1+RAM+BATTERY",
    "0x04 ???",
    "MBC2",
    "MBC2+BATTERY",
    "0x07 ???",
    "ROM+RAM 1",
    "ROM+RAM+BATTERY 1",
    "0x0A ???",
    "MMM01",
    "MMM01+RAM",
    "MMM01+RAM+BATTERY",
    "0x0E ???",
    "MBC3+TIMER+BATTERY",
    "MBC3+TIMER+RAM+BATTERY 2",
    "MBC3",
    "MBC3+RAM 2",
    "MBC3+RAM+BATTERY 2",
    "0x14 ???",
    "0x15 ???",
    "0x16 ???",
    "0x17 ???",
    "0x18 ???",
    "MBC5",
    "MBC5+RAM",
    "MBC5+RAM+BATTERY",
    "MBC5+RUMBLE",
    "MBC5+RUMBLE+RAM",
    "MBC5+RUMBLE+RAM+BATTERY",
    "0x1F ???",
    "MBC6",
    "0x21 ???",
    "MBC7+SENSOR+RUMBLE+RAM+BATTERY",
];

use std::fs::File;
use std::io::Read;

impl Cart {
    pub fn new() -> Cart {
        let empty = String::from("empty");
        Cart {
            filename: empty,
            rom_data: Vec::new(),
            rom_size: 0,
            rom_header: RomHeader::empty_header(),
        }
    }

    pub fn load_cart(&mut self, filename: String) -> Result<(), std::io::Error> {
        self.filename = filename;
        self.rom_header = RomHeader::new(self.filename.clone());
        let mut file = File::open(&self.filename)?;
        file.read_to_end(&mut self.rom_data)?;
        self.rom_size = self.rom_data.len();
        Ok(())
    }

    pub fn cart_old_lic_name(&self) -> String {
        if self.rom_header.lic_code <= 0xA4 {
            return String::from(LIC_CODE[self.rom_header.lic_code as usize]);
        }
        String::from("UNKNOWN")
    }

    pub fn cart_new_lic_name(&self) -> String {
        if self.rom_header.new_lic_code <= 0xA4 {
            return String::from(LIC_CODE[self.rom_header.new_lic_code as usize]);
        }
        String::from("UNKNOWN")
    }

    pub fn cart_type_name(&self) -> String {
        if self.rom_header.cart_type <= 0x22 {
            return String::from(ROM_TYPES[self.rom_header.cart_type as usize]);
        }
        String::from("UNKNOWN")
    }

    fn checksum(&self) -> String {
        let mut checksum: u8 = 0;
        for address in 0x0134..=0x014C {
            checksum = checksum
                .wrapping_sub(self.rom_data[address])
                .wrapping_sub(1);
        }

        if checksum == self.rom_data[0x014D] {
            return String::from("PASS");
        }
        String::from("FAIL")
    }

    pub fn print_data(&self) {
        // println!("{:#?}", self.rom_header);
        println!("Cartridge Loaded");
        println!(
            "\tTitle:    {}",
            String::from_iter(self.rom_header.title.iter())
        );
        println!(
            "\tType:     {} ({})",
            self.rom_header.cart_type,
            self.cart_type_name()
        );
        println!("\tROM Size: {} KB", self.rom_size / 1000);
        println!("\tRAM Size: {}", self.rom_header.ram_size);
        println!(
            "\tLIC Code: {} (Old: {}) (New: {})",
            self.rom_header.lic_code,
            self.cart_old_lic_name(),
            self.cart_new_lic_name()
        );
        println!("\tROM Vers: {}", self.rom_header.version);
        println!(
            "\tChecksum: {} ({})",
            self.rom_header.checksum,
            self.checksum()
        );
    }
}

#[derive(Debug)]
struct RomHeader {
    entry: [u8; 4],
    logo: [u8; 0x30],
    title: [char; 16],
    new_lic_code: u8,
    sgb_flag: u8,
    cart_type: u8,
    rom_size: u8,
    ram_size: u8,
    dest_code: u8,
    lic_code: u8,
    version: u8,
    checksum: u8,
    gbl_checksum: u8,
}

impl RomHeader {
    pub fn empty_header() -> RomHeader {
        RomHeader {
            entry: [0; 4],
            logo: [0; 0x30],
            title: ['\0'; 16],
            new_lic_code: 0,
            sgb_flag: 0,
            cart_type: 0,
            rom_size: 0,
            ram_size: 0,
            dest_code: 0,
            lic_code: 0,
            version: 0,
            checksum: 0,
            gbl_checksum: 0,
        }
    }

    fn new(filename: String) -> RomHeader {
        let mut file = File::open(filename).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let rom_header = RomHeader {
            entry: { buffer[0x0100..=0x0103].try_into().unwrap_or([0, 0, 0, 0]) },
            logo: { buffer[0x0104..=0x0133].try_into().unwrap_or([0; 48]) },
            title: {
                let mut array: [char; 16] = ['\0'; 16];
                for (i, c) in String::from_utf8_lossy(&buffer[0x0134..=0x0143])
                    .chars()
                    .take(16)
                    .enumerate()
                {
                    array[i] = c;
                }
                array
            },
            new_lic_code: {
                let result = match (buffer.get(0x0134), buffer.get(0x0143)) {
                    (Some(&a), Some(&b)) => Some((a << 4) | b),
                    _ => Some(0),
                };
                result.unwrap()
            },
            sgb_flag: buffer[0x0146],
            cart_type: buffer[0x0147],
            rom_size: buffer[0x0148],
            ram_size: buffer[0x0149],
            dest_code: buffer[0x014A],
            lic_code: buffer[0x014B],
            version: buffer[0x014C],
            checksum: buffer[0x014D],
            gbl_checksum: {
                let result = match (buffer.get(0x014E), buffer.get(0x014F)) {
                    (Some(&a), Some(&b)) => Some((a << 4) | b),
                    _ => Some(0),
                };
                result.unwrap()
            },
        };
        rom_header
    }
}

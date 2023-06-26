
pub struct Cart<'a>{
    filename: &'a str,
    rom_size: u32,
    rom_data:  u8,
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


impl Cart<'_> {
    pub fn new() -> Cart<'static>{
        let empty = "empty";
        return Cart{filename: empty, rom_data: 0, rom_size: 0, rom_header: RomHeader::empty_header()};
    }

    pub fn load_cart(&mut self, filename: &str) -> (){
        self.rom_header = RomHeader::new(filename);
    }

    pub fn cart_lic_name<'a>(&self) -> &'a str{
        if self.rom_header.new_lic_code <= 0xA4{
            return LIC_CODE[self.rom_header.lic_code as usize];
        }
        return "unknown";
    }

    pub fn cart_type_name<'a>(&self)-> &'a str{
        if self.rom_header.cart_type <= 0x22 {
            return ROM_TYPES[self.rom_header.cart_type as usize];
        }
        return "UNKNOWN";
    }

    pub fn print_data(&self){
        println!("{:#?}", self.rom_header);
    }
    

}


struct RomHeader{
    entry:          [u8; 4],
    logo:           [u8; 0x30],
    title:          [char; 16],
    new_lic_code:   u8,
    sgb_flag:       u8,
    cart_type:      u8,
    rom_size:       u8,
    ram_size:       u8,
    dest_code:      u8,
    lic_code:       u8,
    version:        u8,
    checksum:       u8,
    gbl_checksum:   u8,
}

use std::fs::File;
use std::io::Read;

impl RomHeader{
    pub fn empty_header() -> RomHeader{
        return RomHeader { entry: [0; 4], logo: [0; 0x30], title: ['\0'; 16], new_lic_code: 0, sgb_flag: 0, cart_type: 0, rom_size: 0, ram_size: 0, dest_code: 0, lic_code: 0, version: 0, checksum: 0, gbl_checksum: 0 }
    }

    fn new(filename: &str) -> RomHeader{

        let mut file = File::open(filename).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
    

        let rom_header = RomHeader{
            entry: {
                let result: [u8; 4] = match buffer[0x0100..=0x0103].try_into(){
                    Ok(slice) => {
                        slice
                    }Err(_) => {
                        [0,0,0,0]
                    }
                };
                result
            },
            logo: {
                let result: [u8; 48] = match buffer[0x0104..=0x0133].try_into(){
                    Ok(slice) => {
                        slice
                    }Err(_) => {
                        [0; 48]
                    }
                };
                result
            },
            title: {
                let mut array: [char; 16] = ['\0'; 16];
                for (i, c) in String::from_utf8_lossy(&buffer[0x0134..=0x0143]).chars().take(16).enumerate(){
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
        
        return rom_header;
    }
}

use std::fmt;
impl fmt::Debug for RomHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RomHeader")
            .field("entry", &self.entry)
            .field("logo", &self.logo)
            .field("title", &self.title)
            .field("new_lic_code", &self.new_lic_code)
            .field("sgb_flag", &self.sgb_flag)
            .field("cart_type", &self.cart_type)
            .field("rom_size", &self.rom_size)
            .field("ram_size", &self.ram_size)
            .field("dest_code", &self.dest_code)
            .field("lic_code", &self.lic_code)
            .field("version", &self.version)
            .field("checksum", &self.checksum)
            .field("gbl_checksum", &self.gbl_checksum)
            .finish()
    }
}
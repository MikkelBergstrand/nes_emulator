use std::usize;

// Returns the pattern table following the standard pattern table
// layout. The pattern tables are laid out next to each other
// horizontally, and each pattern table is a 16x16 grid of tiles.
// Each tile is a 8x8 grid of pixels.
// Each pixel is a 3-tuple of RGB values.
// The layout of the vector is row-first
// Returns bytes, width , and height
pub fn pattern_tables_to_bytes(chr_rom: &[u8]) -> (Vec<u8>, usize, usize) {
    let pattern_table_length = 0x1000 as usize;
    let num_pattern_tables = chr_rom.len() / pattern_table_length;
    let num_tiles = pattern_table_length / 16; // 16 bytes per 8x8 tile

    // Store RGB values per pixel (x3)
    // OR two bytes to get the proper pixel color index (/2)
    // Two pattern tables (x2)
    // Each byte represents a row of pixels (x8)
    let mut colors: Vec<u8> = vec![0u8; pattern_table_length * 8 * 3];

    let width = 128 * num_pattern_tables;
    let height = 128;

    for pattern_table in 0..num_pattern_tables {
        let base_addr = 0x1000*pattern_table;
        for tile in 0..num_tiles {
            let tile_base_x = pattern_table*16 + (tile % 16);
            let tile_base_y = tile / 16;

            for tile_y in 0..8 {
                let addr1 = base_addr + (tile << 4) + tile_y;
                let addr2 = base_addr + (tile << 4) + tile_y + 8;

                let row1 = chr_rom[addr1];
                let row2 = chr_rom[addr2];

                for tile_x in 0..8 {
                    let color_idx_1 = (row1 >> (7 - tile_x)) & 1;
                    let color_idx_2 = (row2 >> (7 - tile_x)) & 1;
                    //
                    // Encode a row of a tile as a byte in the table.
                    // Each row is stored sequentially in the colors table
                    // to form a tile.
                    // Encode each pixel in RGB format
                    let color = match (color_idx_1 << 1) | color_idx_2 {
                        0 => [0, 0, 0],
                        1 => [80, 80, 80],
                        2 => [160, 160, 160],
                        3 => [255, 255, 255],
                        _ => panic!("Invalid color index"),
                    };

                    let idx = width*(tile_base_y*8+tile_y) + tile_base_x*8+tile_x;
                    colors[3*idx + 0] = color[0];
                    colors[3*idx + 1] = color[1];
                    colors[3*idx + 2] = color[2];
                }
            }
        }
    }

    return (colors, width, height);
}

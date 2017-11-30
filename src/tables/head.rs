/// The Font Header Table.  This table gives information about the font,
/// including revision number, truetype flags, creation and modification
/// date, and various gemoetric information.

pub struct Head {
    font_revision: Fixed,
    check_sum_adjustment: u32,
    flags: u16,
    units_per_em: u16,
    created: LONGDATETIME,
    modified: LONGDATETIME,
    x_min: i16,
    y_min: i16,
    x_max: i16,
    y_max: i16,
    mac_style: u16,
    lowest_rec_ppem: u16,
    font_direction_hint: i16,
    index_to_loc_format: i16,
    glyph_data_format: i16,
}
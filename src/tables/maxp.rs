/// Maximum Profile table.
/// This table establishes the memory requirements for a given font.
/// Fonts using CFF data must use version 0.5, where TrueType fonts must use version 1.0

// Fixed	version	0x00005000 for version 0.5
// Fixed	version	0x00010000 for version 1.0.

struct Maxp {
    /// Common to fonts using both TrueType and CFF glyphs.
    num_glyphs: u16,

    /// TrueType fonts additional profile information.
    profile: Option<Profile>,
}


pub struct Profile {
    max_points: u16,
    max_contours: u16,
    max_composite_points: u16,
    max_composite_contours: u16,
    max_zone: u16,
    max_twilight_points: u16,
    max_storage: u16,
    max_function_defs: u16,
    max_instruction_defs: u16,
    max_stack_elements: u16,
    max_size_of_instructions: u16,
    max_component_elements: u16,
    max_component_depth: u16,
}
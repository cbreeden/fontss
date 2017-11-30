/// Horizontal Metrics table.

struct Hmtx<'buf> {
    buffer: &'buf [u8],
}

// Type	Name	Description
// longHorMetric	hMetrics
// [numberOfHMetrics]	Paired advance width and left side bearing values for each glyph. Records are indexed by glyph ID.
// int16	leftSideBearings
// [numGlyphs - numberOfHMetrics]	Left side bearings for glyph IDs greater than or equal to numberOfHMetrics.

pub struct HmtxRecord {
    pub advance_width: u16,
    pub left_side_bearing: u16,
}
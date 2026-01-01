use serde::Deserialize;

/// Determine whether sixel protocol is supported
/// by the terminal running broot.
///
/// This is called only once, and cached in renderer state.
pub fn detect_sixel_protocol_support() -> SixelSupport {
    // TODO: detect by are-we-sixel-yet
    // TODO: should detect by option
    // TODO: detect by CSI
    #[cfg(feature = "kitty-csi-check")]
    {
        let start = std::time::Instant::now();
        const TIMEOUT_MS: u64 = 200;
        let response = xterm_query::query_osc("", TIMEOUT_MS);
        let s = match response {
            Err(e) => {
                debug!("xterm querying failed: {}", e);
                false
            }
            Ok(response) if response = "" => true,
            Ok(_) => false,
        };
        debug!("Xterm querying took {:?}", start.elapsed());
        debug!("kitty protocol support: {:?}", s);
        return s;
    }
    SixelSupport::Unknown
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum SixelSupport {
    None,
    #[default]
    Detect,
    Unknown,
}


mod codec;
mod version;

use tracing::{info, instrument};

use crate::error::Result;
use crate::wire::Envelope;
use crate::world::World;
use crate::{codec as world_codec};

use self::codec::{base64_decode, base64_encode, zlib_deflate, zlib_inflate};
use self::version::{CURRENT_VERSION, check_supported, split_version};

#[instrument(level = "info", skip_all)]
pub fn decode_string(s: &str) -> Result<World> {
    let (version, payload) = split_version(s.trim())?;
    check_supported(version)?;
    let compressed = base64_decode(payload)?;
    let json = zlib_inflate(&compressed)?;
    info!(bytes = json.len(), "decoded blueprint payload");
    let envelope: Envelope = serde_json::from_slice(&json)?;
    world_codec::to_world(envelope)
}

#[instrument(level = "info", skip_all)]
pub fn encode_string(world: &World) -> Result<String> {
    let envelope = world_codec::from_world(world);
    let json = serde_json::to_vec(&envelope)?;
    info!(bytes = json.len(), "encoding blueprint payload");
    let compressed = zlib_deflate(&json)?;
    let mut out = String::with_capacity(compressed.len() * 2);
    out.push(CURRENT_VERSION);
    out.push_str(&base64_encode(&compressed));
    Ok(out)
}

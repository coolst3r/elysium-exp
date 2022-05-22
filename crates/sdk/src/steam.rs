#[repr(C)]
pub struct SteamAPIContext {
    pub steam_client: *const (),
    pub steam_user: *const (),
    pub steam_friends: *const (),
    pub steam_utils: *const (),
}

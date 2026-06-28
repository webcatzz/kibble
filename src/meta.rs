//! Metadata.
//!
//! Metadata is displayed by the system back to the user; for example, an
//! application's name might be displayed in a taskbar. Metadata should be set
//! up as early as possible; it may be updated later, but the system may not
//! change what it displays. All metadata is optional.
//!
//! To update metadata based on the project's `Cargo.toml`, see
//! [`set_from_cargo()`].

use std::ffi::CString;

use sdl3_sys::init::{SDL_SetAppMetadataProperty, SDL_PROP_APP_METADATA_COPYRIGHT_STRING, SDL_PROP_APP_METADATA_CREATOR_STRING, SDL_PROP_APP_METADATA_IDENTIFIER_STRING, SDL_PROP_APP_METADATA_NAME_STRING, SDL_PROP_APP_METADATA_TYPE_STRING, SDL_PROP_APP_METADATA_URL_STRING, SDL_PROP_APP_METADATA_VERSION_STRING};

use crate::sdl_assert;

/// Sets the human-readable name of the application, e.g. "My Game 2: Bad Guy's
/// Revenge".
///
/// If not set, defaults to "SDL Application".
pub fn set_name(name: &str) {
	sdl_assert!(unsafe { SDL_SetAppMetadataProperty(SDL_PROP_APP_METADATA_NAME_STRING, CString::new(name).unwrap().as_ptr()) })
}

/// Sets the version of the application, e.g. "1.0.3beta2", "April 22nd, 2024",
/// or a git hash.
pub fn set_version(version: &str) {
	sdl_assert!(unsafe { SDL_SetAppMetadataProperty(SDL_PROP_APP_METADATA_VERSION_STRING, CString::new(version).unwrap().as_ptr()) })
}

/// Sets the type of the application.
///
/// The type should be be "game" for a video game, "mediaplayer" for a media
/// player, or "application" if nothing else applies. If not set, defaults to
/// "application".
pub fn set_type(t: &str) {
	sdl_assert!(unsafe { SDL_SetAppMetadataProperty(SDL_PROP_APP_METADATA_TYPE_STRING, CString::new(t).unwrap().as_ptr()) })
}

/// Sets the human-readable name(s) of the author(s) of the application, e.g.
/// "MojoWorkshop, LLC".
pub fn set_author(author: &str) {
	sdl_assert!(unsafe { SDL_SetAppMetadataProperty(SDL_PROP_APP_METADATA_CREATOR_STRING, CString::new(author).unwrap().as_ptr()) })
}

/// Sets the human-readable copyright notice of the application, e.g. "Copyright
/// (c) 2024 MojoWorkshop, LLC".
///
/// Notices should not be longer than a line.
pub fn set_copyright(copyright: &str) {
	sdl_assert!(unsafe { SDL_SetAppMetadataProperty(SDL_PROP_APP_METADATA_COPYRIGHT_STRING, CString::new(copyright).unwrap().as_ptr()) })
}

/// Sets the URL associated with the application, e.g. a product page,
/// storefront, or git repository.
pub fn set_webpage(webpage: &str) {
	sdl_assert!(unsafe { SDL_SetAppMetadataProperty(SDL_PROP_APP_METADATA_URL_STRING, CString::new(webpage).unwrap().as_ptr()) })
}

/// Sets the reverse-domain identifier of the application, e.g.
/// "com.example.mygame2".
///
/// The identifier is used by desktop compositors to identify and group windows
/// together, as well as match applications with associated desktop settings and
/// icons.
pub fn set_identifier(identifier: &str) {
	sdl_assert!(unsafe { SDL_SetAppMetadataProperty(SDL_PROP_APP_METADATA_IDENTIFIER_STRING, CString::new(identifier).unwrap().as_ptr()) })
}

/// Sets custom application metadata.
pub fn set_custom(name: &str, value: &str) {
	sdl_assert!(unsafe { SDL_SetAppMetadataProperty(CString::new(name).unwrap().as_ptr(), CString::new(value).unwrap().as_ptr()) })
}
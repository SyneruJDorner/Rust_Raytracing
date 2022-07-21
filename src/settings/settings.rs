use std::sync::{Arc, RwLock};
use configparser::ini::Ini;

#[derive(Default)]
#[allow(non_snake_case)]
pub struct Settings
{
    pub ASPECT_RATIO: f64,
    pub FOV: u32,
    pub IMAGE_WIDTH: u32,
    pub IMAGE_HEIGHT: u32,
    pub SAMPLES_PER_PIXEL: u32,
    pub MAX_DEPTH: u32,
    pub DEBUG_NORMALS: bool,
    pub DEBUG_AABB: bool
}

impl Settings
{
    pub fn load()
    {
        let mut config = Ini::new();
        let map = config.load("settings.ini");

        match map
        {
            Ok(_map) =>
            {
                let fov = config.get("SCREEN", "FOV").unwrap().parse::<u32>().unwrap();
                let image_width = config.get("SCREEN", "IMAGE_WIDTH").unwrap().parse::<u32>().unwrap();
                let image_height = config.get("SCREEN", "IMAGE_HEIGHT").unwrap().parse::<u32>().unwrap();
                let samples_per_pixel = config.get("SCREEN", "SAMPLES_PER_PIXEL").unwrap().parse::<u32>().unwrap();
                let max_depth = config.get("SCREEN", "MAX_DEPTH").unwrap().parse::<u32>().unwrap();
                let aspect_ratio = (image_width as f64 / image_height as f64) as f64;
                let debug_normals = config.get("DEBUG", "DEBUG_NORMALS").unwrap().parse::<bool>().unwrap();
                let debug_aabb = config.get("DEBUG", "DEBUG_AABB").unwrap().parse::<bool>().unwrap();

                let settings = Settings {
                    ASPECT_RATIO: aspect_ratio,
                    FOV: fov,
                    IMAGE_WIDTH: image_width,
                    IMAGE_HEIGHT: image_height,
                    SAMPLES_PER_PIXEL: samples_per_pixel,
                    MAX_DEPTH: max_depth,
                    DEBUG_NORMALS: debug_normals,
                    DEBUG_AABB: debug_aabb
                };
                settings.make_current();
            }
            Err(e) =>
            {
                println!("Error: {}", e);
                std::process::exit(exitcode::CONFIG);
            }
        }
    }

    #[allow(dead_code)]
    pub fn current() -> Arc<Settings>
    {
        return CURRENT_SETTING.with(|c| c.read().unwrap().clone())
    }

    #[allow(dead_code)]
    pub fn get_fov() -> f64
    {
        return Settings::current().FOV as f64
    }

    #[allow(dead_code)]
    pub fn get_image_width() -> u32
    {
        return Settings::current().IMAGE_WIDTH
    }

    #[allow(dead_code)]
    pub fn get_image_height() -> u32
    {
        return Settings::current().IMAGE_HEIGHT
    }

    #[allow(dead_code)]
    pub fn get_samples_per_pixel() -> u32
    {
        return Settings::current().SAMPLES_PER_PIXEL
    }

    #[allow(dead_code)]
    pub fn get_max_depth() -> u32
    {
        return Settings::current().MAX_DEPTH
    }

    #[allow(dead_code)]
    pub fn get_aspect_ratio() -> f64
    {
        return Settings::current().ASPECT_RATIO
    }

    #[allow(dead_code)]
    pub fn get_debug_normals() -> bool
    {
        return Settings::current().DEBUG_NORMALS
    }

    #[allow(dead_code)]
    pub fn get_debug_aabb() -> bool
    {
        return Settings::current().DEBUG_AABB
    }

    #[allow(dead_code)]
    fn make_current(self)
    {
        CURRENT_SETTING.with(|c| *c.write().unwrap() = Arc::new(self))
    }
}

thread_local!
{
    static CURRENT_SETTING: RwLock<Arc<Settings>> = RwLock::new(Default::default());
}



/*
ERROR CODES:
============================================================================================
USAGE (64)	        The command was used	incorrectly, e.g., with	the
			        wrong number	of arguments, a	bad flag, a bad	syntax
			        in a	parameter, or whatever.

DATAERR	(65)        The input data was incorrect	in some	way.  This
                    should only be used for user's data and not system
                    files.

NOINPUT	(66)        An input file (not a	system file) did not exist or
                    was not readable.  This could also include errors
                    like	"No message" to	a mailer (if it	cared to catch
                    it).

NOUSER (67)	        The user specified did not exist.  This might be
                    used	for mail addresses or remote logins.

NOHOST (68)	        The host specified did not exist.  This is used in
                    mail	addresses or network requests.

UNAVAILABLE (69)    A service is	unavailable.  This can occur if	a sup-
                    port	program	or file	does not exist.	 This can also
                    be used as a	catchall message when something	you
                    wanted to do	does not work, but you do not know
                    why.

SOFTWARE (70)	    An internal software	error has been detected.  This
                    should be limited to	non-operating system related
                    errors as possible.

OSERR (71)	        An operating	system error has been detected.	 This
                    is intended to be used for such things as "cannot
                    fork", "cannot create pipe",	or the like.  It in-
                    cludes things like getuid returning a user that
                    does	not exist in the passwd	file.

OSFILE (72)	        Some	system file (e.g., /etc/passwd,
                    /var/run/utx.active,	etc.) does not exist, cannot
                    be opened, or has some sort of error	(e.g., syntax
                    error).

CANTCREAT (73)      A (user specified) output file cannot be created.

IOERR (74)	        An error occurred while doing I/O on	some file.

TEMPFAIL (75)       Temporary failure, indicating something that	is not
                    really an error.  In	sendmail, this means that a
                    mailer (e.g.) could not create a connection,	and
                    the request should be reattempted later.

PROTOCOL (76)       The remote system returned something	that was "not
                    possible" during a protocol exchange.

NOPERM (77)	        You did not have sufficient permission to perform
                    the operation.  This	is not intended	for file sys-
                    tem problems, which should use NOINPUT or
                    CANTCREAT, but rather for	higher level permis-
                    sions.

CONFIG (78)	        Something was found in an unconfigured or miscon-
                    figured state.
============================================================================================
*/

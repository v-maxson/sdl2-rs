use std::{ffi::CStr, marker::PhantomData};
use crate::{sys::*, SdlError, utils::get_sys_error};
use super::{SdlSubsystem, markers::Audio};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AudioDeviceType {
    /// Used to query the list of recording devices.
    Input = 1,

    /// Used to query the list of output devices.
    Output = 0
}

impl SdlSubsystem<Audio> {

    /// Get the number of built-in audio drivers.
    /// 
    /// The returned value will never be negative.
    #[doc(alias = "SDL_GetNumAudioDrivers")]
    pub fn driver_count(&self) -> u32 {
        #[cfg(feature = "log")] debug!("Calling 'SDL_GetNumAudioDrivers'");
        unsafe {
            SDL_GetNumAudioDrivers() as _
        }
    }

    /// Get the name of the audio driver specified by `index`.
    /// 
    /// Valid `index` values are within the range of `0` to `driver_count() - 1`.
    #[doc(alias = "SDL_GetAudioDriver")]
    pub fn get_driver_name(&self, index: u32) -> Option<String> {
        #[cfg(feature = "log")] debug!("Calling 'SDL_GetAudioDriver'");
        let result = unsafe {
            SDL_GetAudioDriver(index as _)
        };

        if !result.is_null() {
            // The result of SDL_GetAudioDriver will never contain unicode characters, 
            // so it is safe to unwrap the result of `CStr::to_str()`.
            Option::Some(unsafe { CStr::from_ptr(result).to_str().unwrap().to_string() })
        } else {
            Option::None
        }
    }

    /// Returns a [`Vec<String>`] containing all available driver names.
    /// 
    /// Note: This slice does not contain the final "dummy" string.
    /// 
    /// Internally, this calls [`driver_count`] and then calls
    /// [`get_driver_name`] in a loop.
    /// 
    /// ### Errors
    /// - [`SdlError::SysError`]
    pub fn driver_names(&self) -> Result<Vec<String>, SdlError> {
        let driver_count = self.driver_count() - 1;
        let mut vec = vec![];

        for i in (0..driver_count) {
            vec.push(self.get_driver_name(i).unwrap())
        }

        Ok(vec)
    }

    /// Get the name of the currently initialized audio driver.
    #[doc(alias = "SDL_GetCurrentAudioDriver")]
    pub fn current_driver_name(&self) -> String {
        #[cfg(feature = "log")] debug!("Calling 'SDL_GetCurrentAudioDriver'");
        let result = unsafe {
            SDL_GetCurrentAudioDriver()
        };

        // `result` should never be null as the only way to call 
        // this function is by initializing the audio subsystem.
        unsafe { CStr::from_ptr(result).to_str().unwrap().to_string() }
    }

    /// Get the number of built-in audio devices.
    /// 
    /// `_device_type` is unused. 
    /// See [`SDL_GetNumAudioDevices`](https://github.com/libsdl-org/SDL/blob/SDL2/include/SDL_audio.h) for more info.
    /// 
    /// Returns [`Option::None`] if the number of built-in audio devices cannot be determined.
    #[doc(alias = "SDL_GetNumAudioDevices")]
    pub fn audio_device_count(&self, _device_type: AudioDeviceType) -> Option<u32> {
        #[cfg(feature = "log")] debug!("Calling 'SDL_GetNumAudioDevices'");
        let result = unsafe {
            SDL_GetNumAudioDevices(_device_type as _)
        };

        match result {
            // -1 indicates that the count is unavailable.
            -1 => Option::None,
            _ => Option::Some(result as _)
        }
    }

    /// Get the name of the audio driver specified by `index`.
    /// 
    /// Valid `index` values are within the range of `0` to `audio_device_count() - 1`.
    /// 
    /// ### Errors 
    /// - [`SdlError::SysError`]
    #[doc(alias = "SDL_GetAudioDeviceName")]
    pub fn get_audio_device_name(&self, index: i32, device_type: AudioDeviceType) -> Result<String, SdlError> {
        let result = unsafe {
            SDL_GetAudioDeviceName(index, device_type as _)
        };

        if !result.is_null() {
            Ok(unsafe { CStr::from_ptr(result).to_str().unwrap().to_string() })
        } else {
            Err(SdlError::SysError(get_sys_error().unwrap()))
        }
    }

    /// This function is not currently unimplemented and will __panic__ if called.
    /// See [`SDL_OpenAudioDevice`](https://github.com/libsdl-org/SDL/blob/SDL2/include/SDL_audio.h).
    #[doc(alias = "SDL_OpenAudioDevice")]
    pub fn open_audio_device(&self) { todo!() }

    /// This function is not currently unimplemented and will __panic__ if called.
    /// See [`SDL_GetAudioDeviceStatus`](https://github.com/libsdl-org/SDL/blob/SDL2/include/SDL_audio.h).
    #[doc(alias = "SDL_GetAudioDeviceStatus")]
    pub fn get_device_status(&self) { todo!() }

    /// This function is not currently unimplemented and will __panic__ if called.
    /// See [`SDL_PauseAudioDevice`](https://github.com/libsdl-org/SDL/blob/SDL2/include/SDL_audio.h).
    #[doc(alias = "SDL_PauseAudioDevice")]
    pub fn pause_device(&self) { todo!() }

    /// This function is not currently unimplemented and will __panic__ if called.
    /// See [`SDL_PauseAudioDevice`](https://github.com/libsdl-org/SDL/blob/SDL2/include/SDL_audio.h).
    #[doc(alias = "SDL_PauseAudioDevice")]
    pub fn unpause_device(&self) { todo!() }

    /// This function is not currently unimplemented and will __panic__ if called.
    /// See [`SDL_LoadWAV`](https://github.com/libsdl-org/SDL/blob/SDL2/include/SDL_audio.h).
    #[doc(alias = "SDL_LoadWAV")]
    pub fn load_wav(&self) { todo!() }

    /// This function is not currently unimplemented and will __panic__ if called.
    /// See [`SDL_ConvertAudio`](https://github.com/libsdl-org/SDL/blob/SDL2/include/SDL_audio.h).
    #[doc(alias = "SDL_ConvertAudio")]
    pub fn convert_audio(&self) { todo!() }

    /// This function is not currently unimplemented and will __panic__ if called.
    /// See [`SDL_NewAudioStream`](https://github.com/libsdl-org/SDL/blob/SDL2/include/SDL_audio.h).
    #[doc(alias = "SDL_NewAudioStream")]
    pub fn audio_stream(&self) { todo!() }

    /// This function is not currently unimplemented and will __panic__ if called.
    /// See [`SDL_AudioStreamPut`](https://github.com/libsdl-org/SDL/blob/SDL2/include/SDL_audio.h).
    #[doc(alias = "SDL_AudioStreamPut")]
    pub fn add_stream_data(&self) { todo!() }

    /// This function is not currently unimplemented and will __panic__ if called.
    /// See [`SDL_AudioStreamGet`](https://github.com/libsdl-org/SDL/blob/SDL2/include/SDL_audio.h).
    #[doc(alias = "SDL_AudioStreamGet")]
    pub fn get_stream_data(&self) { todo!() }

    /// This function is not currently unimplemented and will __panic__ if called.
    /// See [`SDL_AudioStreamAvailable`](https://github.com/libsdl-org/SDL/blob/SDL2/include/SDL_audio.h).
    #[doc(alias = "SDL_AudioStreamAvailable")]
    pub fn get_available_bytes(&self) { todo!() }

    /// This function is not currently unimplemented and will __panic__ if called.
    /// See [`SDL_AudioStreamFlush`](https://github.com/libsdl-org/SDL/blob/SDL2/include/SDL_audio.h).
    #[doc(alias = "SDL_AudioStreamFlush")]
    pub fn flush_stream(&self) { todo!() }

    /// This function is not currently unimplemented and will __panic__ if called.
    /// See [`SDL_AudioStreamClear`](https://github.com/libsdl-org/SDL/blob/SDL2/include/SDL_audio.h).
    #[doc(alias = "SDL_AudioStreamClear")]
    pub fn clear_stream(&self) { todo!() }

    /// This function is not currently unimplemented and will __panic__ if called.
    /// See [`SDL_MixAudioFormat`](https://github.com/libsdl-org/SDL/blob/SDL2/include/SDL_audio.h).
    #[doc(alias = "SDL_MixAudioFormat")]
    pub fn mix_audio_format(&self) { todo!() }

    /// This function is not currently unimplemented and will __panic__ if called.
    /// See [`SDL_QueueAudio`](https://github.com/libsdl-org/SDL/blob/SDL2/include/SDL_audio.h).
    #[doc(alias = "SDL_QueueAudio")]
    pub fn queue_audio(&self) { todo!() }

    /// This function is not currently unimplemented and will __panic__ if called.
    /// See [`SDL_DequeueAudio`](https://github.com/libsdl-org/SDL/blob/SDL2/include/SDL_audio.h).
    #[doc(alias = "SDL_DequeueAudio")]
    pub fn dequeue_audio(&self) { todo!() }

    /// This function is not currently unimplemented and will __panic__ if called.
    /// See [`SDL_GetQueuedAudioSize`](https://github.com/libsdl-org/SDL/blob/SDL2/include/SDL_audio.h).
    #[doc(alias = "SDL_GetQueuedAudioSize")]
    pub fn get_queued_amount(&self) { todo!() }

    /// This function is not currently unimplemented and will __panic__ if called.
    /// See [`SDL_ClearQueuedAudio`](https://github.com/libsdl-org/SDL/blob/SDL2/include/SDL_audio.h).
    #[doc(alias = "SDL_ClearQueuedAudio")]
    pub fn clear_queued_audio(&self) { todo!() }

}

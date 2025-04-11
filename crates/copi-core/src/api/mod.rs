pub mod gpio;
pub mod pio;
pub mod playground;
pub mod pwm;

#[macro_export]
macro_rules! process_common {
    ($state:expr, $req:expr, $msg:expr) => {
        if $req.skip_response {
            $state
                .device_channel
                .send($msg)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Default::default())
        } else {
            let res = $state
                .device_channel
                .fetch($msg)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                .into();

            Ok(Json(res))
        }
    };
}

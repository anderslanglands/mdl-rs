use crate::components::IImageApi;

extern "C" {
    pub fn IImage_api_release(i: IImageApi);
}

pub const NO_ORDER_IMAGES: &[&str] = &[
    "1-ef569229607012ad566a286b258cb0beb41e302e2336021cff96d1a66bba2a6c.jpg",
    "2-0a6201f4743a5aeaedd0eab7e5e33db16ec4b034884be94251cd2cea1226afe3.jpg",
    "3-25dbdf4b05c357fb6023ad7e59e73b84754016f4aa6ea382a6ddfd7279d661ff.jpg",
    "4-b5e14bfb60422031d4763f49bd99c8b0541cf15dadc148723a3e75ad73852612.jpg",
    "5-32e303a90faf1819f45bb8e50da9d9696a3934bf0e9447c2efebc3f505cb7fa8.jpg",
    "6-b489f284ed4d7a82af3ceaf14ccd0410959570b89c151862ff3716d8edd85e01.jpg",
    "7-5854a49a87c718cfee74b7398ea0afdb585793f40899f323db30fba0008467db.jpg",
    "8-bc78ad1859ee746f514df5f2ae5b02e6e3f3b1247cdc78aef54f07507a262c4f.jpg",
    "9-1ba331d7ef2ed48735e8e8de2b1333d714cb2b82a66fef20b680294a692f2496.jpg",
    "10-7af1867a2702aca8911c60d5ce3c054f1a95650906c5982f1942d6bab75150ed.jpg",
    "11-78a1fe8e993961edd5182ee693001567c95e7ab7e51e592762bb62e1e0128437.jpg",
    "12-0c6d646f6255641d4138ef99aa5a4ff1ce7b6a7bb2a51c2d7c031520f904a8bb.jpg",
    "13-d78562a0f5d17619f5f6c35682f3e5bd8a66eb6e5c54853a5f33ca4e811a8f8e.jpg",
    "14-07675dcd1b658e90ee4771d5d25d2b9b1d0dbe2ba474b9f8c1de889be01eeea1.jpg",
    "15-96d3dbdeab9fbf2fadd5d51bd7445279f2a27ddb4e89584cbd0fc3407385ef3d.jpg",
    "16-fdbdef1b0ada3186db05175735bc5e28830c7b7f8976870982b3903528922f00.jpg",
];

pub const ORDERED_IMAGES: &[&str] = &["001.jpg", "002.jpg"];

pub fn no_order_images() -> Vec<String> {
    NO_ORDER_IMAGES.iter().map(|e| String::from(*e)).collect()
}

pub fn ordered_images() -> Vec<String> {
    ORDERED_IMAGES.iter().map(|e| String::from(*e)).collect()
}

mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function(); // will work
    outermost::middle_secret_function(); // won't work
    outermost::inside::inner_function(); // will work
    outermost::inside::secret_function(); // won't work
}

pub mod api {
    pub mod controller;
}
pub mod application {
    pub mod actor {
        pub mod collection;
        pub mod sbom;
    }
    pub mod bus {
        pub mod event;
    }
}
pub mod config;
pub mod domain {
    pub mod collection;
    pub mod component;
}
pub mod infrastructure {
    pub mod generator {
        pub mod sbom {
            pub mod syft;
        }
        pub mod vulnerability {
            pub mod grype;
        }
    }
}

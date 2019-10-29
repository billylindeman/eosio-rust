#[macro_export]
macro_rules! name_type {
    ($ident:ident) => {
        #[derive(
            Debug,
            PartialEq,
            Eq,
            Clone,
            Copy,
            Default,
            Hash,
            PartialOrd,
            Ord,
            serde::Serialize,
            serde::Deserialize,
            crate::bytes::Read,
            crate::bytes::Write,
            crate::bytes::NumBytes,
        )]
        #[__eosio_path = "crate::bytes"]
        pub struct $ident($crate::name::Name);

        impl $ident {
            pub const fn new(value: u64) -> Self {
                Self($crate::name::Name::new(value))
            }

            pub const fn as_u64(&self) -> u64 {
                self.0.as_u64()
            }

            pub const fn as_name(&self) -> $crate::name::Name {
                self.0
            }
        }

        impl std::ops::Deref for $ident {
            type Target = $crate::name::Name;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::convert::AsRef<$crate::name::Name> for $ident {
            fn as_ref(&self) -> &$crate::name::Name {
                &self.0
            }
        }

        impl std::convert::AsRef<$ident> for $ident {
            fn as_ref(&self) -> &Self {
                self
            }
        }

        impl From<u64> for $ident {
            fn from(value: u64) -> Self {
                Self::new(value)
            }
        }

        impl From<$ident> for u64 {
            fn from(value: $ident) -> Self {
                value.as_u64()
            }
        }

        impl From<$crate::name::Name> for $ident {
            fn from(value: $crate::name::Name) -> Self {
                Self(value)
            }
        }

        impl From<$ident> for $crate::name::Name {
            fn from(value: $ident) -> Self {
                value.as_name()
            }
        }

        impl std::str::FromStr for $ident {
            type Err = $crate::name::ParseNameError;
            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let name = $crate::name::Name::from_str(s)?;
                Ok(Self(name))
            }
        }

        impl std::convert::TryFrom<&str> for $ident {
            type Error = $crate::name::ParseNameError;
            #[inline]
            fn try_from(value: &str) -> Result<Self, Self::Error> {
                std::str::FromStr::from_str(value)
            }
        }
    };
}
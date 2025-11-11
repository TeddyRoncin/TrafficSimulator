#[macro_export]
macro_rules! generate_custom_vec {
    ( $Type: ident, $IdxType: ident ) => {
        #[repr(transparent)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $IdxType(pub usize);
        impl std::fmt::Display for $IdxType {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.0.fmt(f) }
        }
        impl std::ops::Index<$IdxType> for Vec<$Type> {
            type Output = $Type;
            fn index(&self, index: $IdxType) -> &Self::Output { &self[index.0] }
        }
        impl std::ops::IndexMut<$IdxType> for Vec<$Type> {
            fn index_mut(&mut self, index: $IdxType) -> &mut Self::Output { &mut self[index.0] }
        }
        impl std::ops::Deref for $IdxType {
            type Target = usize;
            fn deref(&self) -> &Self::Target { &self.0 }
        }
        impl std::ops::DerefMut for $IdxType {
            fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
        }
        impl From<usize> for $IdxType {
            fn from(value: usize) -> Self { $IdxType(value) }
        }
        impl From<$IdxType> for usize {
            fn from(value: $IdxType) -> Self { value.0 }
        }
        impl std::ops::Add<$IdxType> for $IdxType {
            type Output = $IdxType;
            fn add(self, other: $IdxType) -> Self::Output { Self(self.0 + other.0) }
        }

    };
}
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod host {
    use std::ffi::OsStr;
    use std::path::Path;
    use cote::prelude::ASer;
    use cote::prelude::ASet;
    use cote::prelude::Parser;
    use cote::prelude::SetValueFindExt;
    use tokio::io::AsyncWriteExt;
    use tokio::process::Command;
    use wasmtime::component::*;
    use wasmtime_wasi::WasiImpl;
    use wasmtime_wasi::WasiView;
    #[doc(hidden)]
    pub use crate::host::OptSet as __with_name0;
    #[doc(hidden)]
    pub use crate::host::Services as __with_name1;
    /// Auto-generated bindings for a pre-instantiated version of a
    /// component which implements the world `root`.
    ///
    /// This structure is created through [`RootPre::new`] which
    /// takes a [`InstancePre`](wasmtime::component::InstancePre) that
    /// has been created through a [`Linker`](wasmtime::component::Linker).
    pub struct RootPre<T> {
        instance_pre: wasmtime::component::InstancePre<T>,
        interface0: exports::snippet::plugin::compiler::GuestPre,
        interface1: exports::snippet::plugin::language::GuestPre,
    }
    impl<T> Clone for RootPre<T> {
        fn clone(&self) -> Self {
            Self {
                instance_pre: self.instance_pre.clone(),
                interface0: self.interface0.clone(),
                interface1: self.interface1.clone(),
            }
        }
    }
    /// Auto-generated bindings for an instance a component which
    /// implements the world `root`.
    ///
    /// This structure is created through either
    /// [`Root::instantiate_async`] or by first creating
    /// a [`RootPre`] followed by using
    /// [`RootPre::instantiate_async`].
    pub struct Root {
        interface0: exports::snippet::plugin::compiler::Guest,
        interface1: exports::snippet::plugin::language::Guest,
    }
    const _: () = {
        #[allow(unused_imports)]
        use wasmtime::component::__internal::anyhow;
        impl<_T> RootPre<_T> {
            /// Creates a new copy of `RootPre` bindings which can then
            /// be used to instantiate into a particular store.
            ///
            /// This method may fail if the component behind `instance_pre`
            /// does not have the required exports.
            pub fn new(
                instance_pre: wasmtime::component::InstancePre<_T>,
            ) -> wasmtime::Result<Self> {
                let _component = instance_pre.component();
                let interface0 = exports::snippet::plugin::compiler::GuestPre::new(
                    _component,
                )?;
                let interface1 = exports::snippet::plugin::language::GuestPre::new(
                    _component,
                )?;
                Ok(RootPre {
                    instance_pre,
                    interface0,
                    interface1,
                })
            }
            /// Instantiates a new instance of [`Root`] within the
            /// `store` provided.
            ///
            /// This function will use `self` as the pre-instantiated
            /// instance to perform instantiation. Afterwards the preloaded
            /// indices in `self` are used to lookup all exports on the
            /// resulting instance.
            pub async fn instantiate_async(
                &self,
                mut store: impl wasmtime::AsContextMut<Data = _T>,
            ) -> wasmtime::Result<Root>
            where
                _T: Send,
            {
                let mut store = store.as_context_mut();
                let _instance = self.instance_pre.instantiate_async(&mut store).await?;
                let interface0 = self.interface0.load(&mut store, &_instance)?;
                let interface1 = self.interface1.load(&mut store, &_instance)?;
                Ok(Root { interface0, interface1 })
            }
            pub fn engine(&self) -> &wasmtime::Engine {
                self.instance_pre.engine()
            }
            pub fn instance_pre(&self) -> &wasmtime::component::InstancePre<_T> {
                &self.instance_pre
            }
        }
        impl Root {
            /// Convenience wrapper around [`RootPre::new`] and
            /// [`RootPre::instantiate_async`].
            pub async fn instantiate_async<_T>(
                mut store: impl wasmtime::AsContextMut<Data = _T>,
                component: &wasmtime::component::Component,
                linker: &wasmtime::component::Linker<_T>,
            ) -> wasmtime::Result<Root>
            where
                _T: Send,
            {
                let pre = linker.instantiate_pre(component)?;
                RootPre::new(pre)?.instantiate_async(store).await
            }
            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                T: Send,
                U: snippet::plugin::types::Host + Send,
            {
                snippet::plugin::types::add_to_linker(linker, get)?;
                Ok(())
            }
            pub fn snippet_plugin_compiler(
                &self,
            ) -> &exports::snippet::plugin::compiler::Guest {
                &self.interface0
            }
            pub fn snippet_plugin_language(
                &self,
            ) -> &exports::snippet::plugin::language::Guest {
                &self.interface1
            }
        }
    };
    pub mod snippet {
        pub mod plugin {
            #[allow(clippy::all)]
            pub mod types {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::anyhow;
                #[component(enum)]
                pub enum Lang {
                    #[component(name = "c")]
                    C,
                    #[component(name = "cxx")]
                    Cxx,
                    #[component(name = "rust")]
                    Rust,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for Lang {
                    #[inline]
                    fn clone(&self) -> Lang {
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for Lang {}
                #[automatically_derived]
                impl ::core::cmp::Eq for Lang {
                    #[inline]
                    #[doc(hidden)]
                    #[coverage(off)]
                    fn assert_receiver_is_total_eq(&self) -> () {}
                }
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for Lang {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for Lang {
                    #[inline]
                    fn eq(&self, other: &Lang) -> bool {
                        let __self_discr = ::core::intrinsics::discriminant_value(self);
                        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                        __self_discr == __arg1_discr
                    }
                }
                unsafe impl wasmtime::component::Lower for Lang {
                    #[inline]
                    fn lower<T>(
                        &self,
                        cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        dst: &mut core::mem::MaybeUninit<Self::Lower>,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Enum(i) => {
                                &cx.types[i]
                            }
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        match self {
                            Self::C => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::MaybeUninitExt;
                                            let m: &mut core::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(0u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).C)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                            Self::Cxx => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::MaybeUninitExt;
                                            let m: &mut core::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(1u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).Cxx)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                            Self::Rust => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::MaybeUninitExt;
                                            let m: &mut core::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(2u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).Rust)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                        }
                    }
                    #[inline]
                    fn store<T>(
                        &self,
                        cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        mut offset: usize,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Enum(i) => {
                                &cx.types[i]
                            }
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        if true {
                            if !(offset
                                % (<Self as wasmtime::component::ComponentType>::ALIGN32
                                    as usize) == 0)
                            {
                                ::core::panicking::panic(
                                    "assertion failed: offset % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                                )
                            }
                        }
                        match self {
                            Self::C => {
                                *cx.get::<1usize>(offset) = 0u8.to_le_bytes();
                                Ok(())
                            }
                            Self::Cxx => {
                                *cx.get::<1usize>(offset) = 1u8.to_le_bytes();
                                Ok(())
                            }
                            Self::Rust => {
                                *cx.get::<1usize>(offset) = 2u8.to_le_bytes();
                                Ok(())
                            }
                        }
                    }
                }
                unsafe impl wasmtime::component::Lift for Lang {
                    #[inline]
                    fn lift(
                        cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        src: &Self::Lower,
                    ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Enum(i) => {
                                &cx.types[i]
                            }
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        Ok(
                            match src.tag.get_u32() {
                                0u32 => Self::C,
                                1u32 => Self::Cxx,
                                2u32 => Self::Rust,
                                discrim => {
                                    return ::anyhow::__private::Err(
                                        ::anyhow::Error::msg({
                                            let res = ::alloc::fmt::format(
                                                format_args!("unexpected discriminant: {0}", discrim),
                                            );
                                            res
                                        }),
                                    );
                                }
                            },
                        )
                    }
                    #[inline]
                    fn load(
                        cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        bytes: &[u8],
                    ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                        let align = <Self as wasmtime::component::ComponentType>::ALIGN32;
                        if true {
                            if !((bytes.as_ptr() as usize) % (align as usize) == 0) {
                                ::core::panicking::panic(
                                    "assertion failed: (bytes.as_ptr() as usize) % (align as usize) == 0",
                                )
                            }
                        }
                        let discrim = bytes[0];
                        let payload_offset = <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32;
                        let payload = &bytes[payload_offset..];
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Enum(i) => {
                                &cx.types[i]
                            }
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        Ok(
                            match discrim {
                                0u8 => Self::C,
                                1u8 => Self::Cxx,
                                2u8 => Self::Rust,
                                discrim => {
                                    return ::anyhow::__private::Err(
                                        ::anyhow::Error::msg({
                                            let res = ::alloc::fmt::format(
                                                format_args!("unexpected discriminant: {0}", discrim),
                                            );
                                            res
                                        }),
                                    );
                                }
                            },
                        )
                    }
                }
                const _: () = {
                    #[doc(hidden)]
                    #[repr(C)]
                    pub struct LowerLang {
                        tag: wasmtime::ValRaw,
                        payload: LowerPayloadLang,
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for LowerLang {
                        #[inline]
                        fn clone(&self) -> LowerLang {
                            let _: ::core::clone::AssertParamIsClone<wasmtime::ValRaw>;
                            let _: ::core::clone::AssertParamIsClone<LowerPayloadLang>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for LowerLang {}
                    #[doc(hidden)]
                    #[allow(non_snake_case)]
                    #[repr(C)]
                    union LowerPayloadLang {
                        C: [wasmtime::ValRaw; 0],
                        Cxx: [wasmtime::ValRaw; 0],
                        Rust: [wasmtime::ValRaw; 0],
                    }
                    #[automatically_derived]
                    #[allow(non_snake_case)]
                    impl ::core::clone::Clone for LowerPayloadLang {
                        #[inline]
                        fn clone(&self) -> LowerPayloadLang {
                            let _: ::core::clone::AssertParamIsCopy<Self>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    #[allow(non_snake_case)]
                    impl ::core::marker::Copy for LowerPayloadLang {}
                    unsafe impl wasmtime::component::ComponentType for Lang {
                        type Lower = LowerLang;
                        #[inline]
                        fn typecheck(
                            ty: &wasmtime::component::__internal::InterfaceType,
                            types: &wasmtime::component::__internal::InstanceType<'_>,
                        ) -> wasmtime::component::__internal::anyhow::Result<()> {
                            wasmtime::component::__internal::typecheck_enum(
                                ty,
                                types,
                                &["c", "cxx", "rust"],
                            )
                        }
                        const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::variant_static(
                            &[None, None, None],
                        );
                    }
                    unsafe impl wasmtime::component::__internal::ComponentVariant
                    for Lang {
                        const CASES: &'static [Option<
                            wasmtime::component::__internal::CanonicalAbiInfo,
                        >] = &[None, None, None];
                    }
                };
                impl core::fmt::Debug for Lang {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        match self {
                            Lang::C => f.debug_tuple("Lang::C").finish(),
                            Lang::Cxx => f.debug_tuple("Lang::Cxx").finish(),
                            Lang::Rust => f.debug_tuple("Lang::Rust").finish(),
                        }
                    }
                }
                const _: () = {
                    if !(1 == <Lang as wasmtime::component::ComponentType>::SIZE32) {
                        ::core::panicking::panic(
                            "assertion failed: 1 == <Lang as wasmtime::component::ComponentType>::SIZE32",
                        )
                    }
                    if !(1 == <Lang as wasmtime::component::ComponentType>::ALIGN32) {
                        ::core::panicking::panic(
                            "assertion failed: 1 == <Lang as wasmtime::component::ComponentType>::ALIGN32",
                        )
                    }
                };
                #[component(enum)]
                pub enum Mode {
                    /// Compile the code to object
                    #[component(name = "compile")]
                    Compile,
                    /// Expand the macro or do preprocessing
                    #[component(name = "expand")]
                    Expand,
                    /// Compile the code to assembly
                    #[component(name = "assemble")]
                    Assemble,
                    /// Compile the code and link the object to executable
                    #[component(name = "link")]
                    Link,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for Mode {
                    #[inline]
                    fn clone(&self) -> Mode {
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for Mode {}
                #[automatically_derived]
                impl ::core::cmp::Eq for Mode {
                    #[inline]
                    #[doc(hidden)]
                    #[coverage(off)]
                    fn assert_receiver_is_total_eq(&self) -> () {}
                }
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for Mode {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for Mode {
                    #[inline]
                    fn eq(&self, other: &Mode) -> bool {
                        let __self_discr = ::core::intrinsics::discriminant_value(self);
                        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                        __self_discr == __arg1_discr
                    }
                }
                unsafe impl wasmtime::component::Lower for Mode {
                    #[inline]
                    fn lower<T>(
                        &self,
                        cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        dst: &mut core::mem::MaybeUninit<Self::Lower>,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Enum(i) => {
                                &cx.types[i]
                            }
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        match self {
                            Self::Compile => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::MaybeUninitExt;
                                            let m: &mut core::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(0u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).Compile)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                            Self::Expand => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::MaybeUninitExt;
                                            let m: &mut core::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(1u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).Expand)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                            Self::Assemble => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::MaybeUninitExt;
                                            let m: &mut core::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(2u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).Assemble)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                            Self::Link => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::MaybeUninitExt;
                                            let m: &mut core::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(3u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).Link)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                        }
                    }
                    #[inline]
                    fn store<T>(
                        &self,
                        cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        mut offset: usize,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Enum(i) => {
                                &cx.types[i]
                            }
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        if true {
                            if !(offset
                                % (<Self as wasmtime::component::ComponentType>::ALIGN32
                                    as usize) == 0)
                            {
                                ::core::panicking::panic(
                                    "assertion failed: offset % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                                )
                            }
                        }
                        match self {
                            Self::Compile => {
                                *cx.get::<1usize>(offset) = 0u8.to_le_bytes();
                                Ok(())
                            }
                            Self::Expand => {
                                *cx.get::<1usize>(offset) = 1u8.to_le_bytes();
                                Ok(())
                            }
                            Self::Assemble => {
                                *cx.get::<1usize>(offset) = 2u8.to_le_bytes();
                                Ok(())
                            }
                            Self::Link => {
                                *cx.get::<1usize>(offset) = 3u8.to_le_bytes();
                                Ok(())
                            }
                        }
                    }
                }
                unsafe impl wasmtime::component::Lift for Mode {
                    #[inline]
                    fn lift(
                        cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        src: &Self::Lower,
                    ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Enum(i) => {
                                &cx.types[i]
                            }
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        Ok(
                            match src.tag.get_u32() {
                                0u32 => Self::Compile,
                                1u32 => Self::Expand,
                                2u32 => Self::Assemble,
                                3u32 => Self::Link,
                                discrim => {
                                    return ::anyhow::__private::Err(
                                        ::anyhow::Error::msg({
                                            let res = ::alloc::fmt::format(
                                                format_args!("unexpected discriminant: {0}", discrim),
                                            );
                                            res
                                        }),
                                    );
                                }
                            },
                        )
                    }
                    #[inline]
                    fn load(
                        cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        bytes: &[u8],
                    ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                        let align = <Self as wasmtime::component::ComponentType>::ALIGN32;
                        if true {
                            if !((bytes.as_ptr() as usize) % (align as usize) == 0) {
                                ::core::panicking::panic(
                                    "assertion failed: (bytes.as_ptr() as usize) % (align as usize) == 0",
                                )
                            }
                        }
                        let discrim = bytes[0];
                        let payload_offset = <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32;
                        let payload = &bytes[payload_offset..];
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Enum(i) => {
                                &cx.types[i]
                            }
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        Ok(
                            match discrim {
                                0u8 => Self::Compile,
                                1u8 => Self::Expand,
                                2u8 => Self::Assemble,
                                3u8 => Self::Link,
                                discrim => {
                                    return ::anyhow::__private::Err(
                                        ::anyhow::Error::msg({
                                            let res = ::alloc::fmt::format(
                                                format_args!("unexpected discriminant: {0}", discrim),
                                            );
                                            res
                                        }),
                                    );
                                }
                            },
                        )
                    }
                }
                const _: () = {
                    #[doc(hidden)]
                    #[repr(C)]
                    pub struct LowerMode {
                        tag: wasmtime::ValRaw,
                        payload: LowerPayloadMode,
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for LowerMode {
                        #[inline]
                        fn clone(&self) -> LowerMode {
                            let _: ::core::clone::AssertParamIsClone<wasmtime::ValRaw>;
                            let _: ::core::clone::AssertParamIsClone<LowerPayloadMode>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for LowerMode {}
                    #[doc(hidden)]
                    #[allow(non_snake_case)]
                    #[repr(C)]
                    union LowerPayloadMode {
                        Compile: [wasmtime::ValRaw; 0],
                        Expand: [wasmtime::ValRaw; 0],
                        Assemble: [wasmtime::ValRaw; 0],
                        Link: [wasmtime::ValRaw; 0],
                    }
                    #[automatically_derived]
                    #[allow(non_snake_case)]
                    impl ::core::clone::Clone for LowerPayloadMode {
                        #[inline]
                        fn clone(&self) -> LowerPayloadMode {
                            let _: ::core::clone::AssertParamIsCopy<Self>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    #[allow(non_snake_case)]
                    impl ::core::marker::Copy for LowerPayloadMode {}
                    unsafe impl wasmtime::component::ComponentType for Mode {
                        type Lower = LowerMode;
                        #[inline]
                        fn typecheck(
                            ty: &wasmtime::component::__internal::InterfaceType,
                            types: &wasmtime::component::__internal::InstanceType<'_>,
                        ) -> wasmtime::component::__internal::anyhow::Result<()> {
                            wasmtime::component::__internal::typecheck_enum(
                                ty,
                                types,
                                &["compile", "expand", "assemble", "link"],
                            )
                        }
                        const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::variant_static(
                            &[None, None, None, None],
                        );
                    }
                    unsafe impl wasmtime::component::__internal::ComponentVariant
                    for Mode {
                        const CASES: &'static [Option<
                            wasmtime::component::__internal::CanonicalAbiInfo,
                        >] = &[None, None, None, None];
                    }
                };
                impl core::fmt::Debug for Mode {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        match self {
                            Mode::Compile => f.debug_tuple("Mode::Compile").finish(),
                            Mode::Expand => f.debug_tuple("Mode::Expand").finish(),
                            Mode::Assemble => f.debug_tuple("Mode::Assemble").finish(),
                            Mode::Link => f.debug_tuple("Mode::Link").finish(),
                        }
                    }
                }
                const _: () = {
                    if !(1 == <Mode as wasmtime::component::ComponentType>::SIZE32) {
                        ::core::panicking::panic(
                            "assertion failed: 1 == <Mode as wasmtime::component::ComponentType>::SIZE32",
                        )
                    }
                    if !(1 == <Mode as wasmtime::component::ComponentType>::ALIGN32) {
                        ::core::panicking::panic(
                            "assertion failed: 1 == <Mode as wasmtime::component::ComponentType>::ALIGN32",
                        )
                    }
                };
                #[component(enum)]
                pub enum ErrorType {
                    /// Not support given mode
                    #[component(name = "invalid-mode")]
                    InvalidMode,
                    /// Not support given optimization level
                    #[component(name = "invalid-opt-level")]
                    InvalidOptLevel,
                    /// Not support given language standard
                    #[component(name = "invalid-standard")]
                    InvalidStandard,
                    /// Not found given binary
                    #[component(name = "invalid-binary")]
                    InvalidBinary,
                    /// Invalid option string
                    #[component(name = "invalid-optstr")]
                    InvalidOptstr,
                    /// Can not get optset resource from resource table
                    #[component(name = "invalid-optset-resource")]
                    InvalidOptsetResource,
                    /// Can not get services resource from resource table
                    #[component(name = "invalid-services-resource")]
                    InvalidServicesResource,
                    /// Failed read/write command stdin/stdout
                    #[component(name = "command-io-failed")]
                    CommandIoFailed,
                    /// Command need stdin for write
                    #[component(name = "command-need-stdin")]
                    CommandNeedStdin,
                    /// Failed invoke command
                    #[component(name = "command-invoke-failed")]
                    CommandInvokeFailed,
                    /// Command spawn failed
                    #[component(name = "command-spawn-failed")]
                    CommandSpawnFailed,
                    /// Initialize optset failed
                    #[component(name = "create-optset-failed")]
                    CreateOptsetFailed,
                    /// Can not access value of optset
                    #[component(name = "access-value-failed")]
                    AccessValueFailed,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for ErrorType {
                    #[inline]
                    fn clone(&self) -> ErrorType {
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for ErrorType {}
                #[automatically_derived]
                impl ::core::cmp::Eq for ErrorType {
                    #[inline]
                    #[doc(hidden)]
                    #[coverage(off)]
                    fn assert_receiver_is_total_eq(&self) -> () {}
                }
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for ErrorType {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for ErrorType {
                    #[inline]
                    fn eq(&self, other: &ErrorType) -> bool {
                        let __self_discr = ::core::intrinsics::discriminant_value(self);
                        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                        __self_discr == __arg1_discr
                    }
                }
                unsafe impl wasmtime::component::Lower for ErrorType {
                    #[inline]
                    fn lower<T>(
                        &self,
                        cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        dst: &mut core::mem::MaybeUninit<Self::Lower>,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Enum(i) => {
                                &cx.types[i]
                            }
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        match self {
                            Self::InvalidMode => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::MaybeUninitExt;
                                            let m: &mut core::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(0u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).InvalidMode)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                            Self::InvalidOptLevel => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::MaybeUninitExt;
                                            let m: &mut core::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(1u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).InvalidOptLevel)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                            Self::InvalidStandard => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::MaybeUninitExt;
                                            let m: &mut core::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(2u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).InvalidStandard)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                            Self::InvalidBinary => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::MaybeUninitExt;
                                            let m: &mut core::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(3u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).InvalidBinary)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                            Self::InvalidOptstr => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::MaybeUninitExt;
                                            let m: &mut core::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(4u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).InvalidOptstr)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                            Self::InvalidOptsetResource => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::MaybeUninitExt;
                                            let m: &mut core::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(5u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).InvalidOptsetResource)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                            Self::InvalidServicesResource => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::MaybeUninitExt;
                                            let m: &mut core::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(6u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).InvalidServicesResource)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                            Self::CommandIoFailed => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::MaybeUninitExt;
                                            let m: &mut core::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(7u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).CommandIoFailed)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                            Self::CommandNeedStdin => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::MaybeUninitExt;
                                            let m: &mut core::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(8u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).CommandNeedStdin)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                            Self::CommandInvokeFailed => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::MaybeUninitExt;
                                            let m: &mut core::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(9u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).CommandInvokeFailed)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                            Self::CommandSpawnFailed => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::MaybeUninitExt;
                                            let m: &mut core::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(10u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).CommandSpawnFailed)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                            Self::CreateOptsetFailed => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::MaybeUninitExt;
                                            let m: &mut core::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(11u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).CreateOptsetFailed)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                            Self::AccessValueFailed => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::MaybeUninitExt;
                                            let m: &mut core::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(12u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::MaybeUninitExt;
                                                    let m: &mut core::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).AccessValueFailed)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                        }
                    }
                    #[inline]
                    fn store<T>(
                        &self,
                        cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        mut offset: usize,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Enum(i) => {
                                &cx.types[i]
                            }
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        if true {
                            if !(offset
                                % (<Self as wasmtime::component::ComponentType>::ALIGN32
                                    as usize) == 0)
                            {
                                ::core::panicking::panic(
                                    "assertion failed: offset % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                                )
                            }
                        }
                        match self {
                            Self::InvalidMode => {
                                *cx.get::<1usize>(offset) = 0u8.to_le_bytes();
                                Ok(())
                            }
                            Self::InvalidOptLevel => {
                                *cx.get::<1usize>(offset) = 1u8.to_le_bytes();
                                Ok(())
                            }
                            Self::InvalidStandard => {
                                *cx.get::<1usize>(offset) = 2u8.to_le_bytes();
                                Ok(())
                            }
                            Self::InvalidBinary => {
                                *cx.get::<1usize>(offset) = 3u8.to_le_bytes();
                                Ok(())
                            }
                            Self::InvalidOptstr => {
                                *cx.get::<1usize>(offset) = 4u8.to_le_bytes();
                                Ok(())
                            }
                            Self::InvalidOptsetResource => {
                                *cx.get::<1usize>(offset) = 5u8.to_le_bytes();
                                Ok(())
                            }
                            Self::InvalidServicesResource => {
                                *cx.get::<1usize>(offset) = 6u8.to_le_bytes();
                                Ok(())
                            }
                            Self::CommandIoFailed => {
                                *cx.get::<1usize>(offset) = 7u8.to_le_bytes();
                                Ok(())
                            }
                            Self::CommandNeedStdin => {
                                *cx.get::<1usize>(offset) = 8u8.to_le_bytes();
                                Ok(())
                            }
                            Self::CommandInvokeFailed => {
                                *cx.get::<1usize>(offset) = 9u8.to_le_bytes();
                                Ok(())
                            }
                            Self::CommandSpawnFailed => {
                                *cx.get::<1usize>(offset) = 10u8.to_le_bytes();
                                Ok(())
                            }
                            Self::CreateOptsetFailed => {
                                *cx.get::<1usize>(offset) = 11u8.to_le_bytes();
                                Ok(())
                            }
                            Self::AccessValueFailed => {
                                *cx.get::<1usize>(offset) = 12u8.to_le_bytes();
                                Ok(())
                            }
                        }
                    }
                }
                unsafe impl wasmtime::component::Lift for ErrorType {
                    #[inline]
                    fn lift(
                        cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        src: &Self::Lower,
                    ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Enum(i) => {
                                &cx.types[i]
                            }
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        Ok(
                            match src.tag.get_u32() {
                                0u32 => Self::InvalidMode,
                                1u32 => Self::InvalidOptLevel,
                                2u32 => Self::InvalidStandard,
                                3u32 => Self::InvalidBinary,
                                4u32 => Self::InvalidOptstr,
                                5u32 => Self::InvalidOptsetResource,
                                6u32 => Self::InvalidServicesResource,
                                7u32 => Self::CommandIoFailed,
                                8u32 => Self::CommandNeedStdin,
                                9u32 => Self::CommandInvokeFailed,
                                10u32 => Self::CommandSpawnFailed,
                                11u32 => Self::CreateOptsetFailed,
                                12u32 => Self::AccessValueFailed,
                                discrim => {
                                    return ::anyhow::__private::Err(
                                        ::anyhow::Error::msg({
                                            let res = ::alloc::fmt::format(
                                                format_args!("unexpected discriminant: {0}", discrim),
                                            );
                                            res
                                        }),
                                    );
                                }
                            },
                        )
                    }
                    #[inline]
                    fn load(
                        cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        bytes: &[u8],
                    ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                        let align = <Self as wasmtime::component::ComponentType>::ALIGN32;
                        if true {
                            if !((bytes.as_ptr() as usize) % (align as usize) == 0) {
                                ::core::panicking::panic(
                                    "assertion failed: (bytes.as_ptr() as usize) % (align as usize) == 0",
                                )
                            }
                        }
                        let discrim = bytes[0];
                        let payload_offset = <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32;
                        let payload = &bytes[payload_offset..];
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Enum(i) => {
                                &cx.types[i]
                            }
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        Ok(
                            match discrim {
                                0u8 => Self::InvalidMode,
                                1u8 => Self::InvalidOptLevel,
                                2u8 => Self::InvalidStandard,
                                3u8 => Self::InvalidBinary,
                                4u8 => Self::InvalidOptstr,
                                5u8 => Self::InvalidOptsetResource,
                                6u8 => Self::InvalidServicesResource,
                                7u8 => Self::CommandIoFailed,
                                8u8 => Self::CommandNeedStdin,
                                9u8 => Self::CommandInvokeFailed,
                                10u8 => Self::CommandSpawnFailed,
                                11u8 => Self::CreateOptsetFailed,
                                12u8 => Self::AccessValueFailed,
                                discrim => {
                                    return ::anyhow::__private::Err(
                                        ::anyhow::Error::msg({
                                            let res = ::alloc::fmt::format(
                                                format_args!("unexpected discriminant: {0}", discrim),
                                            );
                                            res
                                        }),
                                    );
                                }
                            },
                        )
                    }
                }
                const _: () = {
                    #[doc(hidden)]
                    #[repr(C)]
                    pub struct LowerErrorType {
                        tag: wasmtime::ValRaw,
                        payload: LowerPayloadErrorType,
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for LowerErrorType {
                        #[inline]
                        fn clone(&self) -> LowerErrorType {
                            let _: ::core::clone::AssertParamIsClone<wasmtime::ValRaw>;
                            let _: ::core::clone::AssertParamIsClone<
                                LowerPayloadErrorType,
                            >;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for LowerErrorType {}
                    #[doc(hidden)]
                    #[allow(non_snake_case)]
                    #[repr(C)]
                    union LowerPayloadErrorType {
                        InvalidMode: [wasmtime::ValRaw; 0],
                        InvalidOptLevel: [wasmtime::ValRaw; 0],
                        InvalidStandard: [wasmtime::ValRaw; 0],
                        InvalidBinary: [wasmtime::ValRaw; 0],
                        InvalidOptstr: [wasmtime::ValRaw; 0],
                        InvalidOptsetResource: [wasmtime::ValRaw; 0],
                        InvalidServicesResource: [wasmtime::ValRaw; 0],
                        CommandIoFailed: [wasmtime::ValRaw; 0],
                        CommandNeedStdin: [wasmtime::ValRaw; 0],
                        CommandInvokeFailed: [wasmtime::ValRaw; 0],
                        CommandSpawnFailed: [wasmtime::ValRaw; 0],
                        CreateOptsetFailed: [wasmtime::ValRaw; 0],
                        AccessValueFailed: [wasmtime::ValRaw; 0],
                    }
                    #[automatically_derived]
                    #[allow(non_snake_case)]
                    impl ::core::clone::Clone for LowerPayloadErrorType {
                        #[inline]
                        fn clone(&self) -> LowerPayloadErrorType {
                            let _: ::core::clone::AssertParamIsCopy<Self>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    #[allow(non_snake_case)]
                    impl ::core::marker::Copy for LowerPayloadErrorType {}
                    unsafe impl wasmtime::component::ComponentType for ErrorType {
                        type Lower = LowerErrorType;
                        #[inline]
                        fn typecheck(
                            ty: &wasmtime::component::__internal::InterfaceType,
                            types: &wasmtime::component::__internal::InstanceType<'_>,
                        ) -> wasmtime::component::__internal::anyhow::Result<()> {
                            wasmtime::component::__internal::typecheck_enum(
                                ty,
                                types,
                                &[
                                    "invalid-mode",
                                    "invalid-opt-level",
                                    "invalid-standard",
                                    "invalid-binary",
                                    "invalid-optstr",
                                    "invalid-optset-resource",
                                    "invalid-services-resource",
                                    "command-io-failed",
                                    "command-need-stdin",
                                    "command-invoke-failed",
                                    "command-spawn-failed",
                                    "create-optset-failed",
                                    "access-value-failed",
                                ],
                            )
                        }
                        const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::variant_static(
                            &[
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                            ],
                        );
                    }
                    unsafe impl wasmtime::component::__internal::ComponentVariant
                    for ErrorType {
                        const CASES: &'static [Option<
                            wasmtime::component::__internal::CanonicalAbiInfo,
                        >] = &[
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                        ];
                    }
                };
                impl ErrorType {
                    pub fn name(&self) -> &'static str {
                        match self {
                            ErrorType::InvalidMode => "invalid-mode",
                            ErrorType::InvalidOptLevel => "invalid-opt-level",
                            ErrorType::InvalidStandard => "invalid-standard",
                            ErrorType::InvalidBinary => "invalid-binary",
                            ErrorType::InvalidOptstr => "invalid-optstr",
                            ErrorType::InvalidOptsetResource => "invalid-optset-resource",
                            ErrorType::InvalidServicesResource => {
                                "invalid-services-resource"
                            }
                            ErrorType::CommandIoFailed => "command-io-failed",
                            ErrorType::CommandNeedStdin => "command-need-stdin",
                            ErrorType::CommandInvokeFailed => "command-invoke-failed",
                            ErrorType::CommandSpawnFailed => "command-spawn-failed",
                            ErrorType::CreateOptsetFailed => "create-optset-failed",
                            ErrorType::AccessValueFailed => "access-value-failed",
                        }
                    }
                    pub fn message(&self) -> &'static str {
                        match self {
                            ErrorType::InvalidMode => "Not support given mode",
                            ErrorType::InvalidOptLevel => {
                                "Not support given optimization level"
                            }
                            ErrorType::InvalidStandard => {
                                "Not support given language standard"
                            }
                            ErrorType::InvalidBinary => "Not found given binary",
                            ErrorType::InvalidOptstr => "Invalid option string",
                            ErrorType::InvalidOptsetResource => {
                                "Can not get optset resource from resource table"
                            }
                            ErrorType::InvalidServicesResource => {
                                "Can not get services resource from resource table"
                            }
                            ErrorType::CommandIoFailed => {
                                "Failed read/write command stdin/stdout"
                            }
                            ErrorType::CommandNeedStdin => "Command need stdin for write",
                            ErrorType::CommandInvokeFailed => "Failed invoke command",
                            ErrorType::CommandSpawnFailed => "Command spawn failed",
                            ErrorType::CreateOptsetFailed => "Initialize optset failed",
                            ErrorType::AccessValueFailed => {
                                "Can not access value of optset"
                            }
                        }
                    }
                }
                impl core::fmt::Debug for ErrorType {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        f.debug_struct("ErrorType")
                            .field("code", &(*self as i32))
                            .field("name", &self.name())
                            .field("message", &self.message())
                            .finish()
                    }
                }
                impl core::fmt::Display for ErrorType {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        f.write_fmt(
                            format_args!("{0} (error {1})", self.name(), *self as i32),
                        )
                    }
                }
                impl std::error::Error for ErrorType {}
                const _: () = {
                    if !(1 == <ErrorType as wasmtime::component::ComponentType>::SIZE32)
                    {
                        ::core::panicking::panic(
                            "assertion failed: 1 == <ErrorType as wasmtime::component::ComponentType>::SIZE32",
                        )
                    }
                    if !(1 == <ErrorType as wasmtime::component::ComponentType>::ALIGN32)
                    {
                        ::core::panicking::panic(
                            "assertion failed: 1 == <ErrorType as wasmtime::component::ComponentType>::ALIGN32",
                        )
                    }
                };
                #[component(record)]
                pub struct CmdResult {
                    #[component(name = "out")]
                    pub out: wasmtime::component::__internal::Vec<u8>,
                    #[component(name = "err")]
                    pub err: wasmtime::component::__internal::Vec<u8>,
                    #[component(name = "ret")]
                    pub ret: i32,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for CmdResult {
                    #[inline]
                    fn clone(&self) -> CmdResult {
                        CmdResult {
                            out: ::core::clone::Clone::clone(&self.out),
                            err: ::core::clone::Clone::clone(&self.err),
                            ret: ::core::clone::Clone::clone(&self.ret),
                        }
                    }
                }
                unsafe impl wasmtime::component::Lower for CmdResult {
                    #[inline]
                    fn lower<T>(
                        &self,
                        cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        dst: &mut core::mem::MaybeUninit<Self::Lower>,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Record(i) => {
                                &cx.types[i]
                            }
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        wasmtime::component::Lower::lower(
                            &self.out,
                            cx,
                            ty.fields[0usize].ty,
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::MaybeUninitExt;
                                        let m: &mut core::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).out)
                                    }
                                }
                            },
                        )?;
                        wasmtime::component::Lower::lower(
                            &self.err,
                            cx,
                            ty.fields[1usize].ty,
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::MaybeUninitExt;
                                        let m: &mut core::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).err)
                                    }
                                }
                            },
                        )?;
                        wasmtime::component::Lower::lower(
                            &self.ret,
                            cx,
                            ty.fields[2usize].ty,
                            {
                                #[allow(unused_unsafe)]
                                {
                                    unsafe {
                                        use ::wasmtime::MaybeUninitExt;
                                        let m: &mut core::mem::MaybeUninit<_> = dst;
                                        m.map(|p| &raw mut (*p).ret)
                                    }
                                }
                            },
                        )?;
                        Ok(())
                    }
                    #[inline]
                    fn store<T>(
                        &self,
                        cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        mut offset: usize,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        if true {
                            if !(offset
                                % (<Self as wasmtime::component::ComponentType>::ALIGN32
                                    as usize) == 0)
                            {
                                ::core::panicking::panic(
                                    "assertion failed: offset % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                                )
                            }
                        }
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Record(i) => {
                                &cx.types[i]
                            }
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        wasmtime::component::Lower::store(
                            &self.out,
                            cx,
                            ty.fields[0usize].ty,
                            <wasmtime::component::__internal::Vec<
                                u8,
                            > as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(&mut offset),
                        )?;
                        wasmtime::component::Lower::store(
                            &self.err,
                            cx,
                            ty.fields[1usize].ty,
                            <wasmtime::component::__internal::Vec<
                                u8,
                            > as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(&mut offset),
                        )?;
                        wasmtime::component::Lower::store(
                            &self.ret,
                            cx,
                            ty.fields[2usize].ty,
                            <i32 as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(&mut offset),
                        )?;
                        Ok(())
                    }
                }
                unsafe impl wasmtime::component::Lift for CmdResult {
                    #[inline]
                    fn lift(
                        cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        src: &Self::Lower,
                    ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Record(i) => {
                                &cx.types[i]
                            }
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        Ok(Self {
                            out: <wasmtime::component::__internal::Vec<
                                u8,
                            > as wasmtime::component::Lift>::lift(
                                cx,
                                ty.fields[0usize].ty,
                                &src.out,
                            )?,
                            err: <wasmtime::component::__internal::Vec<
                                u8,
                            > as wasmtime::component::Lift>::lift(
                                cx,
                                ty.fields[1usize].ty,
                                &src.err,
                            )?,
                            ret: <i32 as wasmtime::component::Lift>::lift(
                                cx,
                                ty.fields[2usize].ty,
                                &src.ret,
                            )?,
                        })
                    }
                    #[inline]
                    fn load(
                        cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        bytes: &[u8],
                    ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Record(i) => {
                                &cx.types[i]
                            }
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        if true {
                            if !((bytes.as_ptr() as usize)
                                % (<Self as wasmtime::component::ComponentType>::ALIGN32
                                    as usize) == 0)
                            {
                                ::core::panicking::panic(
                                    "assertion failed: (bytes.as_ptr() as usize) %\n        (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                                )
                            }
                        }
                        let mut offset = 0;
                        Ok(Self {
                            out: <wasmtime::component::__internal::Vec<
                                u8,
                            > as wasmtime::component::Lift>::load(
                                cx,
                                ty.fields[0usize].ty,
                                &bytes[<wasmtime::component::__internal::Vec<
                                    u8,
                                > as wasmtime::component::ComponentType>::ABI
                                    .next_field32_size(
                                        &mut offset,
                                    )..][..<wasmtime::component::__internal::Vec<
                                    u8,
                                > as wasmtime::component::ComponentType>::SIZE32],
                            )?,
                            err: <wasmtime::component::__internal::Vec<
                                u8,
                            > as wasmtime::component::Lift>::load(
                                cx,
                                ty.fields[1usize].ty,
                                &bytes[<wasmtime::component::__internal::Vec<
                                    u8,
                                > as wasmtime::component::ComponentType>::ABI
                                    .next_field32_size(
                                        &mut offset,
                                    )..][..<wasmtime::component::__internal::Vec<
                                    u8,
                                > as wasmtime::component::ComponentType>::SIZE32],
                            )?,
                            ret: <i32 as wasmtime::component::Lift>::load(
                                cx,
                                ty.fields[2usize].ty,
                                &bytes[<i32 as wasmtime::component::ComponentType>::ABI
                                    .next_field32_size(
                                        &mut offset,
                                    )..][..<i32 as wasmtime::component::ComponentType>::SIZE32],
                            )?,
                        })
                    }
                }
                const _: () = {
                    #[doc(hidden)]
                    #[repr(C)]
                    pub struct LowerCmdResult<T0: Copy, T1: Copy, T2: Copy> {
                        out: T0,
                        err: T1,
                        ret: T2,
                        _align: [wasmtime::ValRaw; 0],
                    }
                    #[automatically_derived]
                    impl<
                        T0: ::core::clone::Clone + Copy,
                        T1: ::core::clone::Clone + Copy,
                        T2: ::core::clone::Clone + Copy,
                    > ::core::clone::Clone for LowerCmdResult<T0, T1, T2> {
                        #[inline]
                        fn clone(&self) -> LowerCmdResult<T0, T1, T2> {
                            LowerCmdResult {
                                out: ::core::clone::Clone::clone(&self.out),
                                err: ::core::clone::Clone::clone(&self.err),
                                ret: ::core::clone::Clone::clone(&self.ret),
                                _align: ::core::clone::Clone::clone(&self._align),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<
                        T0: ::core::marker::Copy + Copy,
                        T1: ::core::marker::Copy + Copy,
                        T2: ::core::marker::Copy + Copy,
                    > ::core::marker::Copy for LowerCmdResult<T0, T1, T2> {}
                    unsafe impl wasmtime::component::ComponentType for CmdResult {
                        type Lower = LowerCmdResult<
                            <wasmtime::component::__internal::Vec<
                                u8,
                            > as wasmtime::component::ComponentType>::Lower,
                            <wasmtime::component::__internal::Vec<
                                u8,
                            > as wasmtime::component::ComponentType>::Lower,
                            <i32 as wasmtime::component::ComponentType>::Lower,
                        >;
                        const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::record_static(
                            &[
                                <wasmtime::component::__internal::Vec<
                                    u8,
                                > as wasmtime::component::ComponentType>::ABI,
                                <wasmtime::component::__internal::Vec<
                                    u8,
                                > as wasmtime::component::ComponentType>::ABI,
                                <i32 as wasmtime::component::ComponentType>::ABI,
                            ],
                        );
                        #[inline]
                        fn typecheck(
                            ty: &wasmtime::component::__internal::InterfaceType,
                            types: &wasmtime::component::__internal::InstanceType<'_>,
                        ) -> wasmtime::component::__internal::anyhow::Result<()> {
                            wasmtime::component::__internal::typecheck_record(
                                ty,
                                types,
                                &[
                                    (
                                        "out",
                                        <wasmtime::component::__internal::Vec<
                                            u8,
                                        > as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                    (
                                        "err",
                                        <wasmtime::component::__internal::Vec<
                                            u8,
                                        > as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                    (
                                        "ret",
                                        <i32 as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                ],
                            )
                        }
                    }
                };
                impl core::fmt::Debug for CmdResult {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        f.debug_struct("CmdResult")
                            .field("out", &self.out)
                            .field("err", &self.err)
                            .field("ret", &self.ret)
                            .finish()
                    }
                }
                const _: () = {
                    if !(20 == <CmdResult as wasmtime::component::ComponentType>::SIZE32)
                    {
                        ::core::panicking::panic(
                            "assertion failed: 20 == <CmdResult as wasmtime::component::ComponentType>::SIZE32",
                        )
                    }
                    if !(4 == <CmdResult as wasmtime::component::ComponentType>::ALIGN32)
                    {
                        ::core::panicking::panic(
                            "assertion failed: 4 == <CmdResult as wasmtime::component::ComponentType>::ALIGN32",
                        )
                    }
                };
                pub use super::super::super::__with_name0 as Optset;
                pub trait HostOptset {
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn new<'life0, 'async_trait>(
                        &'life0 mut self,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = wasmtime::component::Resource<Optset>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn default<'life0, 'async_trait>(
                        &'life0 mut self,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<
                                    wasmtime::component::Resource<Optset>,
                                    ErrorType,
                                >,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                    /// Add an option to the option set
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn add_opt<'life0, 'async_trait>(
                        &'life0 mut self,
                        self_: wasmtime::component::Resource<Optset>,
                        opt: wasmtime::component::__internal::String,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<u64, ErrorType>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn get_value_str<'life0, 'async_trait>(
                        &'life0 mut self,
                        self_: wasmtime::component::Resource<Optset>,
                        name: wasmtime::component::__internal::String,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<
                                    wasmtime::component::__internal::String,
                                    ErrorType,
                                >,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                    fn drop(
                        &mut self,
                        rep: wasmtime::component::Resource<Optset>,
                    ) -> wasmtime::Result<()>;
                }
                impl<_T: HostOptset + ?Sized + Send> HostOptset for &mut _T {
                    #[allow(
                        clippy::async_yields_async,
                        clippy::diverging_sub_expression,
                        clippy::let_unit_value,
                        clippy::no_effect_underscore_binding,
                        clippy::shadow_same,
                        clippy::type_complexity,
                        clippy::type_repetition_in_bounds,
                        clippy::used_underscore_binding
                    )]
                    fn new<'life0, 'async_trait>(
                        &'life0 mut self,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = wasmtime::component::Resource<Optset>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait,
                    {
                        Box::pin(async move {
                            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                                wasmtime::component::Resource<Optset>,
                            > {
                                #[allow(unreachable_code)] return __ret;
                            }
                            let mut __self = self;
                            let __ret: wasmtime::component::Resource<Optset> = {
                                HostOptset::new(*__self).await
                            };
                            #[allow(unreachable_code)] __ret
                        })
                    }
                    #[allow(
                        clippy::async_yields_async,
                        clippy::diverging_sub_expression,
                        clippy::let_unit_value,
                        clippy::no_effect_underscore_binding,
                        clippy::shadow_same,
                        clippy::type_complexity,
                        clippy::type_repetition_in_bounds,
                        clippy::used_underscore_binding
                    )]
                    fn default<'life0, 'async_trait>(
                        &'life0 mut self,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<
                                    wasmtime::component::Resource<Optset>,
                                    ErrorType,
                                >,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait,
                    {
                        Box::pin(async move {
                            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                                Result<wasmtime::component::Resource<Optset>, ErrorType>,
                            > {
                                #[allow(unreachable_code)] return __ret;
                            }
                            let mut __self = self;
                            let __ret: Result<
                                wasmtime::component::Resource<Optset>,
                                ErrorType,
                            > = { HostOptset::default(*__self).await };
                            #[allow(unreachable_code)] __ret
                        })
                    }
                    /// Add an option to the option set
                    #[allow(
                        clippy::async_yields_async,
                        clippy::diverging_sub_expression,
                        clippy::let_unit_value,
                        clippy::no_effect_underscore_binding,
                        clippy::shadow_same,
                        clippy::type_complexity,
                        clippy::type_repetition_in_bounds,
                        clippy::used_underscore_binding
                    )]
                    fn add_opt<'life0, 'async_trait>(
                        &'life0 mut self,
                        self_: wasmtime::component::Resource<Optset>,
                        opt: wasmtime::component::__internal::String,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<u64, ErrorType>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait,
                    {
                        Box::pin(async move {
                            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                                Result<u64, ErrorType>,
                            > {
                                #[allow(unreachable_code)] return __ret;
                            }
                            let mut __self = self;
                            let self_ = self_;
                            let opt = opt;
                            let __ret: Result<u64, ErrorType> = {
                                HostOptset::add_opt(*__self, self_, opt).await
                            };
                            #[allow(unreachable_code)] __ret
                        })
                    }
                    #[allow(
                        clippy::async_yields_async,
                        clippy::diverging_sub_expression,
                        clippy::let_unit_value,
                        clippy::no_effect_underscore_binding,
                        clippy::shadow_same,
                        clippy::type_complexity,
                        clippy::type_repetition_in_bounds,
                        clippy::used_underscore_binding
                    )]
                    fn get_value_str<'life0, 'async_trait>(
                        &'life0 mut self,
                        self_: wasmtime::component::Resource<Optset>,
                        name: wasmtime::component::__internal::String,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<
                                    wasmtime::component::__internal::String,
                                    ErrorType,
                                >,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait,
                    {
                        Box::pin(async move {
                            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                                Result<wasmtime::component::__internal::String, ErrorType>,
                            > {
                                #[allow(unreachable_code)] return __ret;
                            }
                            let mut __self = self;
                            let self_ = self_;
                            let name = name;
                            let __ret: Result<
                                wasmtime::component::__internal::String,
                                ErrorType,
                            > = {
                                HostOptset::get_value_str(*__self, self_, name).await
                            };
                            #[allow(unreachable_code)] __ret
                        })
                    }
                    fn drop(
                        &mut self,
                        rep: wasmtime::component::Resource<Optset>,
                    ) -> wasmtime::Result<()> {
                        HostOptset::drop(*self, rep)
                    }
                }
                pub use super::super::super::__with_name1 as Services;
                pub trait HostServices {
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn new<'life0, 'async_trait>(
                        &'life0 mut self,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = wasmtime::component::Resource<Services>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                    /// Is the compiler in debug mode?
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn debug<'life0, 'async_trait>(
                        &'life0 mut self,
                        self_: wasmtime::component::Resource<Services>,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<bool, ErrorType>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                    /// Current language.
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn lang<'life0, 'async_trait>(
                        &'life0 mut self,
                        self_: wasmtime::component::Resource<Services>,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<Lang, ErrorType>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                    /// Current arguments.
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn args<'life0, 'async_trait>(
                        &'life0 mut self,
                        self_: wasmtime::component::Resource<Services>,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<
                                    wasmtime::component::__internal::Vec<
                                        wasmtime::component::__internal::String,
                                    >,
                                    ErrorType,
                                >,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                    /// Current compile mode.
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn mode<'life0, 'async_trait>(
                        &'life0 mut self,
                        self_: wasmtime::component::Resource<Services>,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<Mode, ErrorType>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                    /// Set the language.
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn set_lang<'life0, 'async_trait>(
                        &'life0 mut self,
                        self_: wasmtime::component::Resource<Services>,
                        language: Lang,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<(), ErrorType>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                    /// Set debug mode.
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn set_debug<'life0, 'async_trait>(
                        &'life0 mut self,
                        self_: wasmtime::component::Resource<Services>,
                        debug: bool,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<(), ErrorType>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                    /// Set the compile mode.
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn set_mode<'life0, 'async_trait>(
                        &'life0 mut self,
                        self_: wasmtime::component::Resource<Services>,
                        mode: Mode,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<(), ErrorType>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                    /// Add an argument.
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn add_arg<'life0, 'async_trait>(
                        &'life0 mut self,
                        self_: wasmtime::component::Resource<Services>,
                        arg: wasmtime::component::__internal::String,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<(), ErrorType>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                    /// Append arguments.
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn add_args<'life0, 'async_trait>(
                        &'life0 mut self,
                        self_: wasmtime::component::Resource<Services>,
                        args: wasmtime::component::__internal::Vec<
                            wasmtime::component::__internal::String,
                        >,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<(), ErrorType>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                    /// Find the executable binary.
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn find_bin<'life0, 'async_trait>(
                        &'life0 mut self,
                        bin: wasmtime::component::__internal::String,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<
                                    wasmtime::component::__internal::Vec<u8>,
                                    ErrorType,
                                >,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                    /// Invoke the command
                    #[must_use]
                    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
                    fn invoke_cmd<'life0, 'async_trait>(
                        &'life0 mut self,
                        bin: wasmtime::component::__internal::Vec<u8>,
                        args: wasmtime::component::__internal::Vec<
                            wasmtime::component::__internal::String,
                        >,
                        stdin: wasmtime::component::__internal::Vec<
                            wasmtime::component::__internal::String,
                        >,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<CmdResult, ErrorType>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait;
                    fn drop(
                        &mut self,
                        rep: wasmtime::component::Resource<Services>,
                    ) -> wasmtime::Result<()>;
                }
                impl<_T: HostServices + ?Sized + Send> HostServices for &mut _T {
                    #[allow(
                        clippy::async_yields_async,
                        clippy::diverging_sub_expression,
                        clippy::let_unit_value,
                        clippy::no_effect_underscore_binding,
                        clippy::shadow_same,
                        clippy::type_complexity,
                        clippy::type_repetition_in_bounds,
                        clippy::used_underscore_binding
                    )]
                    fn new<'life0, 'async_trait>(
                        &'life0 mut self,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = wasmtime::component::Resource<Services>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait,
                    {
                        Box::pin(async move {
                            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                                wasmtime::component::Resource<Services>,
                            > {
                                #[allow(unreachable_code)] return __ret;
                            }
                            let mut __self = self;
                            let __ret: wasmtime::component::Resource<Services> = {
                                HostServices::new(*__self).await
                            };
                            #[allow(unreachable_code)] __ret
                        })
                    }
                    /// Is the compiler in debug mode?
                    #[allow(
                        clippy::async_yields_async,
                        clippy::diverging_sub_expression,
                        clippy::let_unit_value,
                        clippy::no_effect_underscore_binding,
                        clippy::shadow_same,
                        clippy::type_complexity,
                        clippy::type_repetition_in_bounds,
                        clippy::used_underscore_binding
                    )]
                    fn debug<'life0, 'async_trait>(
                        &'life0 mut self,
                        self_: wasmtime::component::Resource<Services>,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<bool, ErrorType>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait,
                    {
                        Box::pin(async move {
                            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                                Result<bool, ErrorType>,
                            > {
                                #[allow(unreachable_code)] return __ret;
                            }
                            let mut __self = self;
                            let self_ = self_;
                            let __ret: Result<bool, ErrorType> = {
                                HostServices::debug(*__self, self_).await
                            };
                            #[allow(unreachable_code)] __ret
                        })
                    }
                    /// Current language.
                    #[allow(
                        clippy::async_yields_async,
                        clippy::diverging_sub_expression,
                        clippy::let_unit_value,
                        clippy::no_effect_underscore_binding,
                        clippy::shadow_same,
                        clippy::type_complexity,
                        clippy::type_repetition_in_bounds,
                        clippy::used_underscore_binding
                    )]
                    fn lang<'life0, 'async_trait>(
                        &'life0 mut self,
                        self_: wasmtime::component::Resource<Services>,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<Lang, ErrorType>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait,
                    {
                        Box::pin(async move {
                            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                                Result<Lang, ErrorType>,
                            > {
                                #[allow(unreachable_code)] return __ret;
                            }
                            let mut __self = self;
                            let self_ = self_;
                            let __ret: Result<Lang, ErrorType> = {
                                HostServices::lang(*__self, self_).await
                            };
                            #[allow(unreachable_code)] __ret
                        })
                    }
                    /// Current arguments.
                    #[allow(
                        clippy::async_yields_async,
                        clippy::diverging_sub_expression,
                        clippy::let_unit_value,
                        clippy::no_effect_underscore_binding,
                        clippy::shadow_same,
                        clippy::type_complexity,
                        clippy::type_repetition_in_bounds,
                        clippy::used_underscore_binding
                    )]
                    fn args<'life0, 'async_trait>(
                        &'life0 mut self,
                        self_: wasmtime::component::Resource<Services>,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<
                                    wasmtime::component::__internal::Vec<
                                        wasmtime::component::__internal::String,
                                    >,
                                    ErrorType,
                                >,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait,
                    {
                        Box::pin(async move {
                            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                                Result<
                                    wasmtime::component::__internal::Vec<
                                        wasmtime::component::__internal::String,
                                    >,
                                    ErrorType,
                                >,
                            > {
                                #[allow(unreachable_code)] return __ret;
                            }
                            let mut __self = self;
                            let self_ = self_;
                            let __ret: Result<
                                wasmtime::component::__internal::Vec<
                                    wasmtime::component::__internal::String,
                                >,
                                ErrorType,
                            > = { HostServices::args(*__self, self_).await };
                            #[allow(unreachable_code)] __ret
                        })
                    }
                    /// Current compile mode.
                    #[allow(
                        clippy::async_yields_async,
                        clippy::diverging_sub_expression,
                        clippy::let_unit_value,
                        clippy::no_effect_underscore_binding,
                        clippy::shadow_same,
                        clippy::type_complexity,
                        clippy::type_repetition_in_bounds,
                        clippy::used_underscore_binding
                    )]
                    fn mode<'life0, 'async_trait>(
                        &'life0 mut self,
                        self_: wasmtime::component::Resource<Services>,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<Mode, ErrorType>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait,
                    {
                        Box::pin(async move {
                            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                                Result<Mode, ErrorType>,
                            > {
                                #[allow(unreachable_code)] return __ret;
                            }
                            let mut __self = self;
                            let self_ = self_;
                            let __ret: Result<Mode, ErrorType> = {
                                HostServices::mode(*__self, self_).await
                            };
                            #[allow(unreachable_code)] __ret
                        })
                    }
                    /// Set the language.
                    #[allow(
                        clippy::async_yields_async,
                        clippy::diverging_sub_expression,
                        clippy::let_unit_value,
                        clippy::no_effect_underscore_binding,
                        clippy::shadow_same,
                        clippy::type_complexity,
                        clippy::type_repetition_in_bounds,
                        clippy::used_underscore_binding
                    )]
                    fn set_lang<'life0, 'async_trait>(
                        &'life0 mut self,
                        self_: wasmtime::component::Resource<Services>,
                        language: Lang,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<(), ErrorType>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait,
                    {
                        Box::pin(async move {
                            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                                Result<(), ErrorType>,
                            > {
                                #[allow(unreachable_code)] return __ret;
                            }
                            let mut __self = self;
                            let self_ = self_;
                            let language = language;
                            let __ret: Result<(), ErrorType> = {
                                HostServices::set_lang(*__self, self_, language).await
                            };
                            #[allow(unreachable_code)] __ret
                        })
                    }
                    /// Set debug mode.
                    #[allow(
                        clippy::async_yields_async,
                        clippy::diverging_sub_expression,
                        clippy::let_unit_value,
                        clippy::no_effect_underscore_binding,
                        clippy::shadow_same,
                        clippy::type_complexity,
                        clippy::type_repetition_in_bounds,
                        clippy::used_underscore_binding
                    )]
                    fn set_debug<'life0, 'async_trait>(
                        &'life0 mut self,
                        self_: wasmtime::component::Resource<Services>,
                        debug: bool,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<(), ErrorType>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait,
                    {
                        Box::pin(async move {
                            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                                Result<(), ErrorType>,
                            > {
                                #[allow(unreachable_code)] return __ret;
                            }
                            let mut __self = self;
                            let self_ = self_;
                            let debug = debug;
                            let __ret: Result<(), ErrorType> = {
                                HostServices::set_debug(*__self, self_, debug).await
                            };
                            #[allow(unreachable_code)] __ret
                        })
                    }
                    /// Set the compile mode.
                    #[allow(
                        clippy::async_yields_async,
                        clippy::diverging_sub_expression,
                        clippy::let_unit_value,
                        clippy::no_effect_underscore_binding,
                        clippy::shadow_same,
                        clippy::type_complexity,
                        clippy::type_repetition_in_bounds,
                        clippy::used_underscore_binding
                    )]
                    fn set_mode<'life0, 'async_trait>(
                        &'life0 mut self,
                        self_: wasmtime::component::Resource<Services>,
                        mode: Mode,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<(), ErrorType>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait,
                    {
                        Box::pin(async move {
                            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                                Result<(), ErrorType>,
                            > {
                                #[allow(unreachable_code)] return __ret;
                            }
                            let mut __self = self;
                            let self_ = self_;
                            let mode = mode;
                            let __ret: Result<(), ErrorType> = {
                                HostServices::set_mode(*__self, self_, mode).await
                            };
                            #[allow(unreachable_code)] __ret
                        })
                    }
                    /// Add an argument.
                    #[allow(
                        clippy::async_yields_async,
                        clippy::diverging_sub_expression,
                        clippy::let_unit_value,
                        clippy::no_effect_underscore_binding,
                        clippy::shadow_same,
                        clippy::type_complexity,
                        clippy::type_repetition_in_bounds,
                        clippy::used_underscore_binding
                    )]
                    fn add_arg<'life0, 'async_trait>(
                        &'life0 mut self,
                        self_: wasmtime::component::Resource<Services>,
                        arg: wasmtime::component::__internal::String,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<(), ErrorType>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait,
                    {
                        Box::pin(async move {
                            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                                Result<(), ErrorType>,
                            > {
                                #[allow(unreachable_code)] return __ret;
                            }
                            let mut __self = self;
                            let self_ = self_;
                            let arg = arg;
                            let __ret: Result<(), ErrorType> = {
                                HostServices::add_arg(*__self, self_, arg).await
                            };
                            #[allow(unreachable_code)] __ret
                        })
                    }
                    /// Append arguments.
                    #[allow(
                        clippy::async_yields_async,
                        clippy::diverging_sub_expression,
                        clippy::let_unit_value,
                        clippy::no_effect_underscore_binding,
                        clippy::shadow_same,
                        clippy::type_complexity,
                        clippy::type_repetition_in_bounds,
                        clippy::used_underscore_binding
                    )]
                    fn add_args<'life0, 'async_trait>(
                        &'life0 mut self,
                        self_: wasmtime::component::Resource<Services>,
                        args: wasmtime::component::__internal::Vec<
                            wasmtime::component::__internal::String,
                        >,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<(), ErrorType>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait,
                    {
                        Box::pin(async move {
                            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                                Result<(), ErrorType>,
                            > {
                                #[allow(unreachable_code)] return __ret;
                            }
                            let mut __self = self;
                            let self_ = self_;
                            let args = args;
                            let __ret: Result<(), ErrorType> = {
                                HostServices::add_args(*__self, self_, args).await
                            };
                            #[allow(unreachable_code)] __ret
                        })
                    }
                    /// Find the executable binary.
                    #[allow(
                        clippy::async_yields_async,
                        clippy::diverging_sub_expression,
                        clippy::let_unit_value,
                        clippy::no_effect_underscore_binding,
                        clippy::shadow_same,
                        clippy::type_complexity,
                        clippy::type_repetition_in_bounds,
                        clippy::used_underscore_binding
                    )]
                    fn find_bin<'life0, 'async_trait>(
                        &'life0 mut self,
                        bin: wasmtime::component::__internal::String,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<
                                    wasmtime::component::__internal::Vec<u8>,
                                    ErrorType,
                                >,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait,
                    {
                        Box::pin(async move {
                            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                                Result<wasmtime::component::__internal::Vec<u8>, ErrorType>,
                            > {
                                #[allow(unreachable_code)] return __ret;
                            }
                            let mut __self = self;
                            let bin = bin;
                            let __ret: Result<
                                wasmtime::component::__internal::Vec<u8>,
                                ErrorType,
                            > = { HostServices::find_bin(*__self, bin).await };
                            #[allow(unreachable_code)] __ret
                        })
                    }
                    /// Invoke the command
                    #[allow(
                        clippy::async_yields_async,
                        clippy::diverging_sub_expression,
                        clippy::let_unit_value,
                        clippy::no_effect_underscore_binding,
                        clippy::shadow_same,
                        clippy::type_complexity,
                        clippy::type_repetition_in_bounds,
                        clippy::used_underscore_binding
                    )]
                    fn invoke_cmd<'life0, 'async_trait>(
                        &'life0 mut self,
                        bin: wasmtime::component::__internal::Vec<u8>,
                        args: wasmtime::component::__internal::Vec<
                            wasmtime::component::__internal::String,
                        >,
                        stdin: wasmtime::component::__internal::Vec<
                            wasmtime::component::__internal::String,
                        >,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<CmdResult, ErrorType>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait,
                    {
                        Box::pin(async move {
                            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                                Result<CmdResult, ErrorType>,
                            > {
                                #[allow(unreachable_code)] return __ret;
                            }
                            let mut __self = self;
                            let bin = bin;
                            let args = args;
                            let stdin = stdin;
                            let __ret: Result<CmdResult, ErrorType> = {
                                HostServices::invoke_cmd(*__self, bin, args, stdin).await
                            };
                            #[allow(unreachable_code)] __ret
                        })
                    }
                    fn drop(
                        &mut self,
                        rep: wasmtime::component::Resource<Services>,
                    ) -> wasmtime::Result<()> {
                        HostServices::drop(*self, rep)
                    }
                }
                pub trait Host: Send + HostOptset + HostServices {}
                pub trait GetHost<
                    T,
                >: Fn(T) -> <Self as GetHost<T>>::Host + Send + Sync + Copy + 'static {
                    type Host: Host + Send;
                }
                impl<F, T, O> GetHost<T> for F
                where
                    F: Fn(T) -> O + Send + Sync + Copy + 'static,
                    O: Host + Send,
                {
                    type Host = O;
                }
                pub fn add_to_linker_get_host<T>(
                    linker: &mut wasmtime::component::Linker<T>,
                    host_getter: impl for<'a> GetHost<&'a mut T>,
                ) -> wasmtime::Result<()>
                where
                    T: Send,
                {
                    let mut inst = linker.instance("snippet:plugin/types@0.1.0")?;
                    inst.resource(
                        "optset",
                        wasmtime::component::ResourceType::host::<Optset>(),
                        move |mut store, rep| -> wasmtime::Result<()> {
                            HostOptset::drop(
                                &mut host_getter(store.data_mut()),
                                wasmtime::component::Resource::new_own(rep),
                            )
                        },
                    )?;
                    inst.resource(
                        "services",
                        wasmtime::component::ResourceType::host::<Services>(),
                        move |mut store, rep| -> wasmtime::Result<()> {
                            HostServices::drop(
                                &mut host_getter(store.data_mut()),
                                wasmtime::component::Resource::new_own(rep),
                            )
                        },
                    )?;
                    inst.func_wrap_async(
                        "[constructor]optset",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = HostOptset::new(host).await;
                            Ok((r,))
                        }),
                    )?;
                    inst.func_wrap_async(
                        "[static]optset.default",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = HostOptset::default(host).await;
                            Ok((r,))
                        }),
                    )?;
                    inst.func_wrap_async(
                        "[method]optset.add-opt",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                            ): (
                                wasmtime::component::Resource<Optset>,
                                wasmtime::component::__internal::String,
                            )|
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = HostOptset::add_opt(host, arg0, arg1).await;
                            Ok((r,))
                        }),
                    )?;
                    inst.func_wrap_async(
                        "[method]optset.get-value-str",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                            ): (
                                wasmtime::component::Resource<Optset>,
                                wasmtime::component::__internal::String,
                            )|
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = HostOptset::get_value_str(host, arg0, arg1).await;
                            Ok((r,))
                        }),
                    )?;
                    inst.func_wrap_async(
                        "[constructor]services",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = HostServices::new(host).await;
                            Ok((r,))
                        }),
                    )?;
                    inst.func_wrap_async(
                        "[method]services.debug",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (arg0,): (wasmtime::component::Resource<Services>,)|
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = HostServices::debug(host, arg0).await;
                            Ok((r,))
                        }),
                    )?;
                    inst.func_wrap_async(
                        "[method]services.lang",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (arg0,): (wasmtime::component::Resource<Services>,)|
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = HostServices::lang(host, arg0).await;
                            Ok((r,))
                        }),
                    )?;
                    inst.func_wrap_async(
                        "[method]services.args",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (arg0,): (wasmtime::component::Resource<Services>,)|
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = HostServices::args(host, arg0).await;
                            Ok((r,))
                        }),
                    )?;
                    inst.func_wrap_async(
                        "[method]services.mode",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (arg0,): (wasmtime::component::Resource<Services>,)|
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = HostServices::mode(host, arg0).await;
                            Ok((r,))
                        }),
                    )?;
                    inst.func_wrap_async(
                        "[method]services.set-lang",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                            ): (wasmtime::component::Resource<Services>, Lang)|
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = HostServices::set_lang(host, arg0, arg1).await;
                            Ok((r,))
                        }),
                    )?;
                    inst.func_wrap_async(
                        "[method]services.set-debug",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                            ): (wasmtime::component::Resource<Services>, bool)|
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = HostServices::set_debug(host, arg0, arg1).await;
                            Ok((r,))
                        }),
                    )?;
                    inst.func_wrap_async(
                        "[method]services.set-mode",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                            ): (wasmtime::component::Resource<Services>, Mode)|
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = HostServices::set_mode(host, arg0, arg1).await;
                            Ok((r,))
                        }),
                    )?;
                    inst.func_wrap_async(
                        "[method]services.add-arg",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                            ): (
                                wasmtime::component::Resource<Services>,
                                wasmtime::component::__internal::String,
                            )|
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = HostServices::add_arg(host, arg0, arg1).await;
                            Ok((r,))
                        }),
                    )?;
                    inst.func_wrap_async(
                        "[method]services.add-args",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                            ): (
                                wasmtime::component::Resource<Services>,
                                wasmtime::component::__internal::Vec<
                                    wasmtime::component::__internal::String,
                                >,
                            )|
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = HostServices::add_args(host, arg0, arg1).await;
                            Ok((r,))
                        }),
                    )?;
                    inst.func_wrap_async(
                        "[static]services.find-bin",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (arg0,): (wasmtime::component::__internal::String,)|
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = HostServices::find_bin(host, arg0).await;
                            Ok((r,))
                        }),
                    )?;
                    inst.func_wrap_async(
                        "[static]services.invoke-cmd",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                                arg2,
                            ): (
                                wasmtime::component::__internal::Vec<u8>,
                                wasmtime::component::__internal::Vec<
                                    wasmtime::component::__internal::String,
                                >,
                                wasmtime::component::__internal::Vec<
                                    wasmtime::component::__internal::String,
                                >,
                            )|
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = HostServices::invoke_cmd(host, arg0, arg1, arg2)
                                .await;
                            Ok((r,))
                        }),
                    )?;
                    Ok(())
                }
                pub fn add_to_linker<T, U>(
                    linker: &mut wasmtime::component::Linker<T>,
                    get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
                ) -> wasmtime::Result<()>
                where
                    U: Host + Send,
                    T: Send,
                {
                    add_to_linker_get_host(linker, get)
                }
                impl<_T: Host + ?Sized + Send> Host for &mut _T {}
            }
        }
    }
    pub mod exports {
        pub mod snippet {
            pub mod plugin {
                #[allow(clippy::all)]
                pub mod compiler {
                    #[allow(unused_imports)]
                    use wasmtime::component::__internal::anyhow;
                    pub type ErrorType = super::super::super::super::snippet::plugin::types::ErrorType;
                    const _: () = {
                        if !(1
                            == <ErrorType as wasmtime::component::ComponentType>::SIZE32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 1 == <ErrorType as wasmtime::component::ComponentType>::SIZE32",
                            )
                        }
                        if !(1
                            == <ErrorType as wasmtime::component::ComponentType>::ALIGN32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 1 == <ErrorType as wasmtime::component::ComponentType>::ALIGN32",
                            )
                        }
                    };
                    pub type Mode = super::super::super::super::snippet::plugin::types::Mode;
                    const _: () = {
                        if !(1 == <Mode as wasmtime::component::ComponentType>::SIZE32) {
                            ::core::panicking::panic(
                                "assertion failed: 1 == <Mode as wasmtime::component::ComponentType>::SIZE32",
                            )
                        }
                        if !(1 == <Mode as wasmtime::component::ComponentType>::ALIGN32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 1 == <Mode as wasmtime::component::ComponentType>::ALIGN32",
                            )
                        }
                    };
                    pub type Lang = super::super::super::super::snippet::plugin::types::Lang;
                    const _: () = {
                        if !(1 == <Lang as wasmtime::component::ComponentType>::SIZE32) {
                            ::core::panicking::panic(
                                "assertion failed: 1 == <Lang as wasmtime::component::ComponentType>::SIZE32",
                            )
                        }
                        if !(1 == <Lang as wasmtime::component::ComponentType>::ALIGN32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 1 == <Lang as wasmtime::component::ComponentType>::ALIGN32",
                            )
                        }
                    };
                    pub type CmdResult = super::super::super::super::snippet::plugin::types::CmdResult;
                    const _: () = {
                        if !(20
                            == <CmdResult as wasmtime::component::ComponentType>::SIZE32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 20 == <CmdResult as wasmtime::component::ComponentType>::SIZE32",
                            )
                        }
                        if !(4
                            == <CmdResult as wasmtime::component::ComponentType>::ALIGN32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 4 == <CmdResult as wasmtime::component::ComponentType>::ALIGN32",
                            )
                        }
                    };
                    pub type Compiler = wasmtime::component::ResourceAny;
                    pub struct GuestCompiler<'a> {
                        funcs: &'a Guest,
                    }
                    pub struct Guest {
                        bin: wasmtime::component::Func,
                        support: wasmtime::component::Func,
                        constructor_compiler_constructor: wasmtime::component::Func,
                        method_compiler_args: wasmtime::component::Func,
                        method_compiler_debug: wasmtime::component::Func,
                        method_compiler_mode: wasmtime::component::Func,
                        method_compiler_set_debug: wasmtime::component::Func,
                        method_compiler_set_mode: wasmtime::component::Func,
                        method_compiler_set_opt_level: wasmtime::component::Func,
                        method_compiler_set_standard: wasmtime::component::Func,
                        method_compiler_add_macro: wasmtime::component::Func,
                        method_compiler_add_include_path: wasmtime::component::Func,
                        method_compiler_add_library_path: wasmtime::component::Func,
                        method_compiler_link_library: wasmtime::component::Func,
                        method_compiler_add_arg: wasmtime::component::Func,
                        method_compiler_add_args: wasmtime::component::Func,
                        method_compiler_compile_code: wasmtime::component::Func,
                        method_compiler_compile_file: wasmtime::component::Func,
                        method_compiler_link_object: wasmtime::component::Func,
                    }
                    pub struct GuestPre {
                        bin: wasmtime::component::ComponentExportIndex,
                        support: wasmtime::component::ComponentExportIndex,
                        constructor_compiler_constructor: wasmtime::component::ComponentExportIndex,
                        method_compiler_args: wasmtime::component::ComponentExportIndex,
                        method_compiler_debug: wasmtime::component::ComponentExportIndex,
                        method_compiler_mode: wasmtime::component::ComponentExportIndex,
                        method_compiler_set_debug: wasmtime::component::ComponentExportIndex,
                        method_compiler_set_mode: wasmtime::component::ComponentExportIndex,
                        method_compiler_set_opt_level: wasmtime::component::ComponentExportIndex,
                        method_compiler_set_standard: wasmtime::component::ComponentExportIndex,
                        method_compiler_add_macro: wasmtime::component::ComponentExportIndex,
                        method_compiler_add_include_path: wasmtime::component::ComponentExportIndex,
                        method_compiler_add_library_path: wasmtime::component::ComponentExportIndex,
                        method_compiler_link_library: wasmtime::component::ComponentExportIndex,
                        method_compiler_add_arg: wasmtime::component::ComponentExportIndex,
                        method_compiler_add_args: wasmtime::component::ComponentExportIndex,
                        method_compiler_compile_code: wasmtime::component::ComponentExportIndex,
                        method_compiler_compile_file: wasmtime::component::ComponentExportIndex,
                        method_compiler_link_object: wasmtime::component::ComponentExportIndex,
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for GuestPre {
                        #[inline]
                        fn clone(&self) -> GuestPre {
                            GuestPre {
                                bin: ::core::clone::Clone::clone(&self.bin),
                                support: ::core::clone::Clone::clone(&self.support),
                                constructor_compiler_constructor: ::core::clone::Clone::clone(
                                    &self.constructor_compiler_constructor,
                                ),
                                method_compiler_args: ::core::clone::Clone::clone(
                                    &self.method_compiler_args,
                                ),
                                method_compiler_debug: ::core::clone::Clone::clone(
                                    &self.method_compiler_debug,
                                ),
                                method_compiler_mode: ::core::clone::Clone::clone(
                                    &self.method_compiler_mode,
                                ),
                                method_compiler_set_debug: ::core::clone::Clone::clone(
                                    &self.method_compiler_set_debug,
                                ),
                                method_compiler_set_mode: ::core::clone::Clone::clone(
                                    &self.method_compiler_set_mode,
                                ),
                                method_compiler_set_opt_level: ::core::clone::Clone::clone(
                                    &self.method_compiler_set_opt_level,
                                ),
                                method_compiler_set_standard: ::core::clone::Clone::clone(
                                    &self.method_compiler_set_standard,
                                ),
                                method_compiler_add_macro: ::core::clone::Clone::clone(
                                    &self.method_compiler_add_macro,
                                ),
                                method_compiler_add_include_path: ::core::clone::Clone::clone(
                                    &self.method_compiler_add_include_path,
                                ),
                                method_compiler_add_library_path: ::core::clone::Clone::clone(
                                    &self.method_compiler_add_library_path,
                                ),
                                method_compiler_link_library: ::core::clone::Clone::clone(
                                    &self.method_compiler_link_library,
                                ),
                                method_compiler_add_arg: ::core::clone::Clone::clone(
                                    &self.method_compiler_add_arg,
                                ),
                                method_compiler_add_args: ::core::clone::Clone::clone(
                                    &self.method_compiler_add_args,
                                ),
                                method_compiler_compile_code: ::core::clone::Clone::clone(
                                    &self.method_compiler_compile_code,
                                ),
                                method_compiler_compile_file: ::core::clone::Clone::clone(
                                    &self.method_compiler_compile_file,
                                ),
                                method_compiler_link_object: ::core::clone::Clone::clone(
                                    &self.method_compiler_link_object,
                                ),
                            }
                        }
                    }
                    impl GuestPre {
                        pub fn new(
                            component: &wasmtime::component::Component,
                        ) -> wasmtime::Result<GuestPre> {
                            let _component = component;
                            let (_, instance) = component
                                .export_index(None, "snippet:plugin/compiler@0.1.0")
                                .ok_or_else(|| ::anyhow::__private::must_use({
                                    let error = ::anyhow::__private::format_err(
                                        format_args!(
                                            "no exported instance named `snippet:plugin/compiler@0.1.0`",
                                        ),
                                    );
                                    error
                                }))?;
                            let _lookup = |name: &str| {
                                _component
                                    .export_index(Some(&instance), name)
                                    .map(|p| p.1)
                                    .ok_or_else(|| {
                                        ::anyhow::__private::must_use({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!(
                                                    "instance export `snippet:plugin/compiler@0.1.0` does not have export `{0}`",
                                                    name,
                                                ),
                                            );
                                            error
                                        })
                                    })
                            };
                            let bin = _lookup("bin")?;
                            let support = _lookup("support")?;
                            let constructor_compiler_constructor = _lookup(
                                "[constructor]compiler",
                            )?;
                            let method_compiler_args = _lookup("[method]compiler.args")?;
                            let method_compiler_debug = _lookup(
                                "[method]compiler.debug",
                            )?;
                            let method_compiler_mode = _lookup("[method]compiler.mode")?;
                            let method_compiler_set_debug = _lookup(
                                "[method]compiler.set-debug",
                            )?;
                            let method_compiler_set_mode = _lookup(
                                "[method]compiler.set-mode",
                            )?;
                            let method_compiler_set_opt_level = _lookup(
                                "[method]compiler.set-opt-level",
                            )?;
                            let method_compiler_set_standard = _lookup(
                                "[method]compiler.set-standard",
                            )?;
                            let method_compiler_add_macro = _lookup(
                                "[method]compiler.add-macro",
                            )?;
                            let method_compiler_add_include_path = _lookup(
                                "[method]compiler.add-include-path",
                            )?;
                            let method_compiler_add_library_path = _lookup(
                                "[method]compiler.add-library-path",
                            )?;
                            let method_compiler_link_library = _lookup(
                                "[method]compiler.link-library",
                            )?;
                            let method_compiler_add_arg = _lookup(
                                "[method]compiler.add-arg",
                            )?;
                            let method_compiler_add_args = _lookup(
                                "[method]compiler.add-args",
                            )?;
                            let method_compiler_compile_code = _lookup(
                                "[method]compiler.compile-code",
                            )?;
                            let method_compiler_compile_file = _lookup(
                                "[method]compiler.compile-file",
                            )?;
                            let method_compiler_link_object = _lookup(
                                "[method]compiler.link-object",
                            )?;
                            Ok(GuestPre {
                                bin,
                                support,
                                constructor_compiler_constructor,
                                method_compiler_args,
                                method_compiler_debug,
                                method_compiler_mode,
                                method_compiler_set_debug,
                                method_compiler_set_mode,
                                method_compiler_set_opt_level,
                                method_compiler_set_standard,
                                method_compiler_add_macro,
                                method_compiler_add_include_path,
                                method_compiler_add_library_path,
                                method_compiler_link_library,
                                method_compiler_add_arg,
                                method_compiler_add_args,
                                method_compiler_compile_code,
                                method_compiler_compile_file,
                                method_compiler_link_object,
                            })
                        }
                        pub fn load(
                            &self,
                            mut store: impl wasmtime::AsContextMut,
                            instance: &wasmtime::component::Instance,
                        ) -> wasmtime::Result<Guest> {
                            let mut store = store.as_context_mut();
                            let _ = &mut store;
                            let _instance = instance;
                            let bin = *_instance
                                .get_typed_func::<
                                    (),
                                    (
                                        Result<wasmtime::component::__internal::Vec<u8>, ErrorType>,
                                    ),
                                >(&mut store, &self.bin)?
                                .func();
                            let support = *_instance
                                .get_typed_func::<(), (Lang,)>(&mut store, &self.support)?
                                .func();
                            let constructor_compiler_constructor = *_instance
                                .get_typed_func::<
                                    (),
                                    (wasmtime::component::ResourceAny,),
                                >(&mut store, &self.constructor_compiler_constructor)?
                                .func();
                            let method_compiler_args = *_instance
                                .get_typed_func::<
                                    (wasmtime::component::ResourceAny,),
                                    (
                                        Result<
                                            wasmtime::component::__internal::Vec<
                                                wasmtime::component::__internal::String,
                                            >,
                                            ErrorType,
                                        >,
                                    ),
                                >(&mut store, &self.method_compiler_args)?
                                .func();
                            let method_compiler_debug = *_instance
                                .get_typed_func::<
                                    (wasmtime::component::ResourceAny,),
                                    (Result<bool, ErrorType>,),
                                >(&mut store, &self.method_compiler_debug)?
                                .func();
                            let method_compiler_mode = *_instance
                                .get_typed_func::<
                                    (wasmtime::component::ResourceAny,),
                                    (Result<Mode, ErrorType>,),
                                >(&mut store, &self.method_compiler_mode)?
                                .func();
                            let method_compiler_set_debug = *_instance
                                .get_typed_func::<
                                    (wasmtime::component::ResourceAny, bool),
                                    (Result<(), ErrorType>,),
                                >(&mut store, &self.method_compiler_set_debug)?
                                .func();
                            let method_compiler_set_mode = *_instance
                                .get_typed_func::<
                                    (wasmtime::component::ResourceAny, Mode),
                                    (Result<(), ErrorType>,),
                                >(&mut store, &self.method_compiler_set_mode)?
                                .func();
                            let method_compiler_set_opt_level = *_instance
                                .get_typed_func::<
                                    (wasmtime::component::ResourceAny, u8),
                                    (Result<(), ErrorType>,),
                                >(&mut store, &self.method_compiler_set_opt_level)?
                                .func();
                            let method_compiler_set_standard = *_instance
                                .get_typed_func::<
                                    (wasmtime::component::ResourceAny, &str),
                                    (Result<(), ErrorType>,),
                                >(&mut store, &self.method_compiler_set_standard)?
                                .func();
                            let method_compiler_add_macro = *_instance
                                .get_typed_func::<
                                    (wasmtime::component::ResourceAny, &str, Option<&str>),
                                    (Result<(), ErrorType>,),
                                >(&mut store, &self.method_compiler_add_macro)?
                                .func();
                            let method_compiler_add_include_path = *_instance
                                .get_typed_func::<
                                    (wasmtime::component::ResourceAny, &str),
                                    (Result<(), ErrorType>,),
                                >(&mut store, &self.method_compiler_add_include_path)?
                                .func();
                            let method_compiler_add_library_path = *_instance
                                .get_typed_func::<
                                    (wasmtime::component::ResourceAny, &str),
                                    (Result<(), ErrorType>,),
                                >(&mut store, &self.method_compiler_add_library_path)?
                                .func();
                            let method_compiler_link_library = *_instance
                                .get_typed_func::<
                                    (wasmtime::component::ResourceAny, &str),
                                    (Result<(), ErrorType>,),
                                >(&mut store, &self.method_compiler_link_library)?
                                .func();
                            let method_compiler_add_arg = *_instance
                                .get_typed_func::<
                                    (wasmtime::component::ResourceAny, &str),
                                    (Result<(), ErrorType>,),
                                >(&mut store, &self.method_compiler_add_arg)?
                                .func();
                            let method_compiler_add_args = *_instance
                                .get_typed_func::<
                                    (
                                        wasmtime::component::ResourceAny,
                                        &[wasmtime::component::__internal::String],
                                    ),
                                    (Result<(), ErrorType>,),
                                >(&mut store, &self.method_compiler_add_args)?
                                .func();
                            let method_compiler_compile_code = *_instance
                                .get_typed_func::<
                                    (
                                        wasmtime::component::ResourceAny,
                                        &[wasmtime::component::__internal::String],
                                        &str,
                                    ),
                                    (Result<CmdResult, ErrorType>,),
                                >(&mut store, &self.method_compiler_compile_code)?
                                .func();
                            let method_compiler_compile_file = *_instance
                                .get_typed_func::<
                                    (wasmtime::component::ResourceAny, &str, &str),
                                    (Result<CmdResult, ErrorType>,),
                                >(&mut store, &self.method_compiler_compile_file)?
                                .func();
                            let method_compiler_link_object = *_instance
                                .get_typed_func::<
                                    (
                                        wasmtime::component::ResourceAny,
                                        &[wasmtime::component::__internal::String],
                                        &str,
                                    ),
                                    (Result<CmdResult, ErrorType>,),
                                >(&mut store, &self.method_compiler_link_object)?
                                .func();
                            Ok(Guest {
                                bin,
                                support,
                                constructor_compiler_constructor,
                                method_compiler_args,
                                method_compiler_debug,
                                method_compiler_mode,
                                method_compiler_set_debug,
                                method_compiler_set_mode,
                                method_compiler_set_opt_level,
                                method_compiler_set_standard,
                                method_compiler_add_macro,
                                method_compiler_add_include_path,
                                method_compiler_add_library_path,
                                method_compiler_link_library,
                                method_compiler_add_arg,
                                method_compiler_add_args,
                                method_compiler_compile_code,
                                method_compiler_compile_file,
                                method_compiler_link_object,
                            })
                        }
                    }
                    impl Guest {
                        /// the PathBuf of binary
                        pub async fn call_bin<S: wasmtime::AsContextMut>(
                            &self,
                            mut store: S,
                        ) -> wasmtime::Result<
                            Result<wasmtime::component::__internal::Vec<u8>, ErrorType>,
                        >
                        where
                            <S as wasmtime::AsContext>::Data: Send,
                        {
                            let callee = unsafe {
                                wasmtime::component::TypedFunc::<
                                    (),
                                    (
                                        Result<wasmtime::component::__internal::Vec<u8>, ErrorType>,
                                    ),
                                >::new_unchecked(self.bin)
                            };
                            let (ret0,) = callee
                                .call_async(store.as_context_mut(), ())
                                .await?;
                            callee.post_return_async(store.as_context_mut()).await?;
                            Ok(ret0)
                        }
                        pub async fn call_support<S: wasmtime::AsContextMut>(
                            &self,
                            mut store: S,
                        ) -> wasmtime::Result<Lang>
                        where
                            <S as wasmtime::AsContext>::Data: Send,
                        {
                            let callee = unsafe {
                                wasmtime::component::TypedFunc::<
                                    (),
                                    (Lang,),
                                >::new_unchecked(self.support)
                            };
                            let (ret0,) = callee
                                .call_async(store.as_context_mut(), ())
                                .await?;
                            callee.post_return_async(store.as_context_mut()).await?;
                            Ok(ret0)
                        }
                        pub fn compiler(&self) -> GuestCompiler<'_> {
                            GuestCompiler { funcs: self }
                        }
                    }
                    impl GuestCompiler<'_> {
                        pub async fn call_constructor<S: wasmtime::AsContextMut>(
                            &self,
                            mut store: S,
                        ) -> wasmtime::Result<wasmtime::component::ResourceAny>
                        where
                            <S as wasmtime::AsContext>::Data: Send,
                        {
                            let callee = unsafe {
                                wasmtime::component::TypedFunc::<
                                    (),
                                    (wasmtime::component::ResourceAny,),
                                >::new_unchecked(
                                    self.funcs.constructor_compiler_constructor,
                                )
                            };
                            let (ret0,) = callee
                                .call_async(store.as_context_mut(), ())
                                .await?;
                            callee.post_return_async(store.as_context_mut()).await?;
                            Ok(ret0)
                        }
                        /// Current arguments.
                        pub async fn call_args<S: wasmtime::AsContextMut>(
                            &self,
                            mut store: S,
                            arg0: wasmtime::component::ResourceAny,
                        ) -> wasmtime::Result<
                            Result<
                                wasmtime::component::__internal::Vec<
                                    wasmtime::component::__internal::String,
                                >,
                                ErrorType,
                            >,
                        >
                        where
                            <S as wasmtime::AsContext>::Data: Send,
                        {
                            let callee = unsafe {
                                wasmtime::component::TypedFunc::<
                                    (wasmtime::component::ResourceAny,),
                                    (
                                        Result<
                                            wasmtime::component::__internal::Vec<
                                                wasmtime::component::__internal::String,
                                            >,
                                            ErrorType,
                                        >,
                                    ),
                                >::new_unchecked(self.funcs.method_compiler_args)
                            };
                            let (ret0,) = callee
                                .call_async(store.as_context_mut(), (arg0,))
                                .await?;
                            callee.post_return_async(store.as_context_mut()).await?;
                            Ok(ret0)
                        }
                        /// Is the compiler in debug mode?
                        pub async fn call_debug<S: wasmtime::AsContextMut>(
                            &self,
                            mut store: S,
                            arg0: wasmtime::component::ResourceAny,
                        ) -> wasmtime::Result<Result<bool, ErrorType>>
                        where
                            <S as wasmtime::AsContext>::Data: Send,
                        {
                            let callee = unsafe {
                                wasmtime::component::TypedFunc::<
                                    (wasmtime::component::ResourceAny,),
                                    (Result<bool, ErrorType>,),
                                >::new_unchecked(self.funcs.method_compiler_debug)
                            };
                            let (ret0,) = callee
                                .call_async(store.as_context_mut(), (arg0,))
                                .await?;
                            callee.post_return_async(store.as_context_mut()).await?;
                            Ok(ret0)
                        }
                        /// Current compile mode.
                        pub async fn call_mode<S: wasmtime::AsContextMut>(
                            &self,
                            mut store: S,
                            arg0: wasmtime::component::ResourceAny,
                        ) -> wasmtime::Result<Result<Mode, ErrorType>>
                        where
                            <S as wasmtime::AsContext>::Data: Send,
                        {
                            let callee = unsafe {
                                wasmtime::component::TypedFunc::<
                                    (wasmtime::component::ResourceAny,),
                                    (Result<Mode, ErrorType>,),
                                >::new_unchecked(self.funcs.method_compiler_mode)
                            };
                            let (ret0,) = callee
                                .call_async(store.as_context_mut(), (arg0,))
                                .await?;
                            callee.post_return_async(store.as_context_mut()).await?;
                            Ok(ret0)
                        }
                        /// Set debug mode.
                        pub async fn call_set_debug<S: wasmtime::AsContextMut>(
                            &self,
                            mut store: S,
                            arg0: wasmtime::component::ResourceAny,
                            arg1: bool,
                        ) -> wasmtime::Result<Result<(), ErrorType>>
                        where
                            <S as wasmtime::AsContext>::Data: Send,
                        {
                            let callee = unsafe {
                                wasmtime::component::TypedFunc::<
                                    (wasmtime::component::ResourceAny, bool),
                                    (Result<(), ErrorType>,),
                                >::new_unchecked(self.funcs.method_compiler_set_debug)
                            };
                            let (ret0,) = callee
                                .call_async(store.as_context_mut(), (arg0, arg1))
                                .await?;
                            callee.post_return_async(store.as_context_mut()).await?;
                            Ok(ret0)
                        }
                        /// Set the compile mode.
                        pub async fn call_set_mode<S: wasmtime::AsContextMut>(
                            &self,
                            mut store: S,
                            arg0: wasmtime::component::ResourceAny,
                            arg1: Mode,
                        ) -> wasmtime::Result<Result<(), ErrorType>>
                        where
                            <S as wasmtime::AsContext>::Data: Send,
                        {
                            let callee = unsafe {
                                wasmtime::component::TypedFunc::<
                                    (wasmtime::component::ResourceAny, Mode),
                                    (Result<(), ErrorType>,),
                                >::new_unchecked(self.funcs.method_compiler_set_mode)
                            };
                            let (ret0,) = callee
                                .call_async(store.as_context_mut(), (arg0, arg1))
                                .await?;
                            callee.post_return_async(store.as_context_mut()).await?;
                            Ok(ret0)
                        }
                        /// Set the optimize level.
                        pub async fn call_set_opt_level<S: wasmtime::AsContextMut>(
                            &self,
                            mut store: S,
                            arg0: wasmtime::component::ResourceAny,
                            arg1: u8,
                        ) -> wasmtime::Result<Result<(), ErrorType>>
                        where
                            <S as wasmtime::AsContext>::Data: Send,
                        {
                            let callee = unsafe {
                                wasmtime::component::TypedFunc::<
                                    (wasmtime::component::ResourceAny, u8),
                                    (Result<(), ErrorType>,),
                                >::new_unchecked(self.funcs.method_compiler_set_opt_level)
                            };
                            let (ret0,) = callee
                                .call_async(store.as_context_mut(), (arg0, arg1))
                                .await?;
                            callee.post_return_async(store.as_context_mut()).await?;
                            Ok(ret0)
                        }
                        /// Set the language standard.
                        pub async fn call_set_standard<S: wasmtime::AsContextMut>(
                            &self,
                            mut store: S,
                            arg0: wasmtime::component::ResourceAny,
                            arg1: &str,
                        ) -> wasmtime::Result<Result<(), ErrorType>>
                        where
                            <S as wasmtime::AsContext>::Data: Send,
                        {
                            let callee = unsafe {
                                wasmtime::component::TypedFunc::<
                                    (wasmtime::component::ResourceAny, &str),
                                    (Result<(), ErrorType>,),
                                >::new_unchecked(self.funcs.method_compiler_set_standard)
                            };
                            let (ret0,) = callee
                                .call_async(store.as_context_mut(), (arg0, arg1))
                                .await?;
                            callee.post_return_async(store.as_context_mut()).await?;
                            Ok(ret0)
                        }
                        /// Add a macro.
                        pub async fn call_add_macro<S: wasmtime::AsContextMut>(
                            &self,
                            mut store: S,
                            arg0: wasmtime::component::ResourceAny,
                            arg1: &str,
                            arg2: Option<&str>,
                        ) -> wasmtime::Result<Result<(), ErrorType>>
                        where
                            <S as wasmtime::AsContext>::Data: Send,
                        {
                            let callee = unsafe {
                                wasmtime::component::TypedFunc::<
                                    (wasmtime::component::ResourceAny, &str, Option<&str>),
                                    (Result<(), ErrorType>,),
                                >::new_unchecked(self.funcs.method_compiler_add_macro)
                            };
                            let (ret0,) = callee
                                .call_async(store.as_context_mut(), (arg0, arg1, arg2))
                                .await?;
                            callee.post_return_async(store.as_context_mut()).await?;
                            Ok(ret0)
                        }
                        /// Add include path.
                        pub async fn call_add_include_path<S: wasmtime::AsContextMut>(
                            &self,
                            mut store: S,
                            arg0: wasmtime::component::ResourceAny,
                            arg1: &str,
                        ) -> wasmtime::Result<Result<(), ErrorType>>
                        where
                            <S as wasmtime::AsContext>::Data: Send,
                        {
                            let callee = unsafe {
                                wasmtime::component::TypedFunc::<
                                    (wasmtime::component::ResourceAny, &str),
                                    (Result<(), ErrorType>,),
                                >::new_unchecked(
                                    self.funcs.method_compiler_add_include_path,
                                )
                            };
                            let (ret0,) = callee
                                .call_async(store.as_context_mut(), (arg0, arg1))
                                .await?;
                            callee.post_return_async(store.as_context_mut()).await?;
                            Ok(ret0)
                        }
                        /// Add library path.
                        pub async fn call_add_library_path<S: wasmtime::AsContextMut>(
                            &self,
                            mut store: S,
                            arg0: wasmtime::component::ResourceAny,
                            arg1: &str,
                        ) -> wasmtime::Result<Result<(), ErrorType>>
                        where
                            <S as wasmtime::AsContext>::Data: Send,
                        {
                            let callee = unsafe {
                                wasmtime::component::TypedFunc::<
                                    (wasmtime::component::ResourceAny, &str),
                                    (Result<(), ErrorType>,),
                                >::new_unchecked(
                                    self.funcs.method_compiler_add_library_path,
                                )
                            };
                            let (ret0,) = callee
                                .call_async(store.as_context_mut(), (arg0, arg1))
                                .await?;
                            callee.post_return_async(store.as_context_mut()).await?;
                            Ok(ret0)
                        }
                        /// Link a library.
                        pub async fn call_link_library<S: wasmtime::AsContextMut>(
                            &self,
                            mut store: S,
                            arg0: wasmtime::component::ResourceAny,
                            arg1: &str,
                        ) -> wasmtime::Result<Result<(), ErrorType>>
                        where
                            <S as wasmtime::AsContext>::Data: Send,
                        {
                            let callee = unsafe {
                                wasmtime::component::TypedFunc::<
                                    (wasmtime::component::ResourceAny, &str),
                                    (Result<(), ErrorType>,),
                                >::new_unchecked(self.funcs.method_compiler_link_library)
                            };
                            let (ret0,) = callee
                                .call_async(store.as_context_mut(), (arg0, arg1))
                                .await?;
                            callee.post_return_async(store.as_context_mut()).await?;
                            Ok(ret0)
                        }
                        /// Add an argument.
                        pub async fn call_add_arg<S: wasmtime::AsContextMut>(
                            &self,
                            mut store: S,
                            arg0: wasmtime::component::ResourceAny,
                            arg1: &str,
                        ) -> wasmtime::Result<Result<(), ErrorType>>
                        where
                            <S as wasmtime::AsContext>::Data: Send,
                        {
                            let callee = unsafe {
                                wasmtime::component::TypedFunc::<
                                    (wasmtime::component::ResourceAny, &str),
                                    (Result<(), ErrorType>,),
                                >::new_unchecked(self.funcs.method_compiler_add_arg)
                            };
                            let (ret0,) = callee
                                .call_async(store.as_context_mut(), (arg0, arg1))
                                .await?;
                            callee.post_return_async(store.as_context_mut()).await?;
                            Ok(ret0)
                        }
                        /// Append arguments.
                        pub async fn call_add_args<S: wasmtime::AsContextMut>(
                            &self,
                            mut store: S,
                            arg0: wasmtime::component::ResourceAny,
                            arg1: &[wasmtime::component::__internal::String],
                        ) -> wasmtime::Result<Result<(), ErrorType>>
                        where
                            <S as wasmtime::AsContext>::Data: Send,
                        {
                            let callee = unsafe {
                                wasmtime::component::TypedFunc::<
                                    (
                                        wasmtime::component::ResourceAny,
                                        &[wasmtime::component::__internal::String],
                                    ),
                                    (Result<(), ErrorType>,),
                                >::new_unchecked(self.funcs.method_compiler_add_args)
                            };
                            let (ret0,) = callee
                                .call_async(store.as_context_mut(), (arg0, arg1))
                                .await?;
                            callee.post_return_async(store.as_context_mut()).await?;
                            Ok(ret0)
                        }
                        /// Compile the code
                        pub async fn call_compile_code<S: wasmtime::AsContextMut>(
                            &self,
                            mut store: S,
                            arg0: wasmtime::component::ResourceAny,
                            arg1: &[wasmtime::component::__internal::String],
                            arg2: &str,
                        ) -> wasmtime::Result<Result<CmdResult, ErrorType>>
                        where
                            <S as wasmtime::AsContext>::Data: Send,
                        {
                            let callee = unsafe {
                                wasmtime::component::TypedFunc::<
                                    (
                                        wasmtime::component::ResourceAny,
                                        &[wasmtime::component::__internal::String],
                                        &str,
                                    ),
                                    (Result<CmdResult, ErrorType>,),
                                >::new_unchecked(self.funcs.method_compiler_compile_code)
                            };
                            let (ret0,) = callee
                                .call_async(store.as_context_mut(), (arg0, arg1, arg2))
                                .await?;
                            callee.post_return_async(store.as_context_mut()).await?;
                            Ok(ret0)
                        }
                        /// Compile the file
                        pub async fn call_compile_file<S: wasmtime::AsContextMut>(
                            &self,
                            mut store: S,
                            arg0: wasmtime::component::ResourceAny,
                            arg1: &str,
                            arg2: &str,
                        ) -> wasmtime::Result<Result<CmdResult, ErrorType>>
                        where
                            <S as wasmtime::AsContext>::Data: Send,
                        {
                            let callee = unsafe {
                                wasmtime::component::TypedFunc::<
                                    (wasmtime::component::ResourceAny, &str, &str),
                                    (Result<CmdResult, ErrorType>,),
                                >::new_unchecked(self.funcs.method_compiler_compile_file)
                            };
                            let (ret0,) = callee
                                .call_async(store.as_context_mut(), (arg0, arg1, arg2))
                                .await?;
                            callee.post_return_async(store.as_context_mut()).await?;
                            Ok(ret0)
                        }
                        /// Link the object into executable
                        pub async fn call_link_object<S: wasmtime::AsContextMut>(
                            &self,
                            mut store: S,
                            arg0: wasmtime::component::ResourceAny,
                            arg1: &[wasmtime::component::__internal::String],
                            arg2: &str,
                        ) -> wasmtime::Result<Result<CmdResult, ErrorType>>
                        where
                            <S as wasmtime::AsContext>::Data: Send,
                        {
                            let callee = unsafe {
                                wasmtime::component::TypedFunc::<
                                    (
                                        wasmtime::component::ResourceAny,
                                        &[wasmtime::component::__internal::String],
                                        &str,
                                    ),
                                    (Result<CmdResult, ErrorType>,),
                                >::new_unchecked(self.funcs.method_compiler_link_object)
                            };
                            let (ret0,) = callee
                                .call_async(store.as_context_mut(), (arg0, arg1, arg2))
                                .await?;
                            callee.post_return_async(store.as_context_mut()).await?;
                            Ok(ret0)
                        }
                    }
                }
                #[allow(clippy::all)]
                pub mod language {
                    #[allow(unused_imports)]
                    use wasmtime::component::__internal::anyhow;
                    pub type ErrorType = super::super::super::super::snippet::plugin::types::ErrorType;
                    const _: () = {
                        if !(1
                            == <ErrorType as wasmtime::component::ComponentType>::SIZE32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 1 == <ErrorType as wasmtime::component::ComponentType>::SIZE32",
                            )
                        }
                        if !(1
                            == <ErrorType as wasmtime::component::ComponentType>::ALIGN32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 1 == <ErrorType as wasmtime::component::ComponentType>::ALIGN32",
                            )
                        }
                    };
                    pub type Optset = super::super::super::super::snippet::plugin::types::Optset;
                    pub type Lang = super::super::super::super::snippet::plugin::types::Lang;
                    const _: () = {
                        if !(1 == <Lang as wasmtime::component::ComponentType>::SIZE32) {
                            ::core::panicking::panic(
                                "assertion failed: 1 == <Lang as wasmtime::component::ComponentType>::SIZE32",
                            )
                        }
                        if !(1 == <Lang as wasmtime::component::ComponentType>::ALIGN32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 1 == <Lang as wasmtime::component::ComponentType>::ALIGN32",
                            )
                        }
                    };
                    pub type CmdResult = super::super::super::super::snippet::plugin::types::CmdResult;
                    const _: () = {
                        if !(20
                            == <CmdResult as wasmtime::component::ComponentType>::SIZE32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 20 == <CmdResult as wasmtime::component::ComponentType>::SIZE32",
                            )
                        }
                        if !(4
                            == <CmdResult as wasmtime::component::ComponentType>::ALIGN32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 4 == <CmdResult as wasmtime::component::ComponentType>::ALIGN32",
                            )
                        }
                    };
                    pub type Compiler = super::super::super::super::exports::snippet::plugin::compiler::Compiler;
                    pub struct Guest {
                        name: wasmtime::component::Func,
                        initialize_optset: wasmtime::component::Func,
                        fill_optset: wasmtime::component::Func,
                        compile: wasmtime::component::Func,
                    }
                    pub struct GuestPre {
                        name: wasmtime::component::ComponentExportIndex,
                        initialize_optset: wasmtime::component::ComponentExportIndex,
                        fill_optset: wasmtime::component::ComponentExportIndex,
                        compile: wasmtime::component::ComponentExportIndex,
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for GuestPre {
                        #[inline]
                        fn clone(&self) -> GuestPre {
                            GuestPre {
                                name: ::core::clone::Clone::clone(&self.name),
                                initialize_optset: ::core::clone::Clone::clone(
                                    &self.initialize_optset,
                                ),
                                fill_optset: ::core::clone::Clone::clone(&self.fill_optset),
                                compile: ::core::clone::Clone::clone(&self.compile),
                            }
                        }
                    }
                    impl GuestPre {
                        pub fn new(
                            component: &wasmtime::component::Component,
                        ) -> wasmtime::Result<GuestPre> {
                            let _component = component;
                            let (_, instance) = component
                                .export_index(None, "snippet:plugin/language@0.1.0")
                                .ok_or_else(|| ::anyhow::__private::must_use({
                                    let error = ::anyhow::__private::format_err(
                                        format_args!(
                                            "no exported instance named `snippet:plugin/language@0.1.0`",
                                        ),
                                    );
                                    error
                                }))?;
                            let _lookup = |name: &str| {
                                _component
                                    .export_index(Some(&instance), name)
                                    .map(|p| p.1)
                                    .ok_or_else(|| {
                                        ::anyhow::__private::must_use({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!(
                                                    "instance export `snippet:plugin/language@0.1.0` does not have export `{0}`",
                                                    name,
                                                ),
                                            );
                                            error
                                        })
                                    })
                            };
                            let name = _lookup("name")?;
                            let initialize_optset = _lookup("initialize-optset")?;
                            let fill_optset = _lookup("fill-optset")?;
                            let compile = _lookup("compile")?;
                            Ok(GuestPre {
                                name,
                                initialize_optset,
                                fill_optset,
                                compile,
                            })
                        }
                        pub fn load(
                            &self,
                            mut store: impl wasmtime::AsContextMut,
                            instance: &wasmtime::component::Instance,
                        ) -> wasmtime::Result<Guest> {
                            let mut store = store.as_context_mut();
                            let _ = &mut store;
                            let _instance = instance;
                            let name = *_instance
                                .get_typed_func::<(), (Lang,)>(&mut store, &self.name)?
                                .func();
                            let initialize_optset = *_instance
                                .get_typed_func::<
                                    (),
                                    (Result<wasmtime::component::Resource<Optset>, ErrorType>,),
                                >(&mut store, &self.initialize_optset)?
                                .func();
                            let fill_optset = *_instance
                                .get_typed_func::<
                                    (wasmtime::component::Resource<Optset>,),
                                    (Result<(), ErrorType>,),
                                >(&mut store, &self.fill_optset)?
                                .func();
                            let compile = *_instance
                                .get_typed_func::<
                                    (
                                        wasmtime::component::Resource<Optset>,
                                        wasmtime::component::ResourceAny,
                                    ),
                                    (Result<CmdResult, ErrorType>,),
                                >(&mut store, &self.compile)?
                                .func();
                            Ok(Guest {
                                name,
                                initialize_optset,
                                fill_optset,
                                compile,
                            })
                        }
                    }
                    impl Guest {
                        pub async fn call_name<S: wasmtime::AsContextMut>(
                            &self,
                            mut store: S,
                        ) -> wasmtime::Result<Lang>
                        where
                            <S as wasmtime::AsContext>::Data: Send,
                        {
                            let callee = unsafe {
                                wasmtime::component::TypedFunc::<
                                    (),
                                    (Lang,),
                                >::new_unchecked(self.name)
                            };
                            let (ret0,) = callee
                                .call_async(store.as_context_mut(), ())
                                .await?;
                            callee.post_return_async(store.as_context_mut()).await?;
                            Ok(ret0)
                        }
                        pub async fn call_initialize_optset<S: wasmtime::AsContextMut>(
                            &self,
                            mut store: S,
                        ) -> wasmtime::Result<
                            Result<wasmtime::component::Resource<Optset>, ErrorType>,
                        >
                        where
                            <S as wasmtime::AsContext>::Data: Send,
                        {
                            let callee = unsafe {
                                wasmtime::component::TypedFunc::<
                                    (),
                                    (Result<wasmtime::component::Resource<Optset>, ErrorType>,),
                                >::new_unchecked(self.initialize_optset)
                            };
                            let (ret0,) = callee
                                .call_async(store.as_context_mut(), ())
                                .await?;
                            callee.post_return_async(store.as_context_mut()).await?;
                            Ok(ret0)
                        }
                        pub async fn call_fill_optset<S: wasmtime::AsContextMut>(
                            &self,
                            mut store: S,
                            arg0: wasmtime::component::Resource<Optset>,
                        ) -> wasmtime::Result<Result<(), ErrorType>>
                        where
                            <S as wasmtime::AsContext>::Data: Send,
                        {
                            let callee = unsafe {
                                wasmtime::component::TypedFunc::<
                                    (wasmtime::component::Resource<Optset>,),
                                    (Result<(), ErrorType>,),
                                >::new_unchecked(self.fill_optset)
                            };
                            let (ret0,) = callee
                                .call_async(store.as_context_mut(), (arg0,))
                                .await?;
                            callee.post_return_async(store.as_context_mut()).await?;
                            Ok(ret0)
                        }
                        pub async fn call_compile<S: wasmtime::AsContextMut>(
                            &self,
                            mut store: S,
                            arg0: wasmtime::component::Resource<Optset>,
                            arg1: wasmtime::component::ResourceAny,
                        ) -> wasmtime::Result<Result<CmdResult, ErrorType>>
                        where
                            <S as wasmtime::AsContext>::Data: Send,
                        {
                            let callee = unsafe {
                                wasmtime::component::TypedFunc::<
                                    (
                                        wasmtime::component::Resource<Optset>,
                                        wasmtime::component::ResourceAny,
                                    ),
                                    (Result<CmdResult, ErrorType>,),
                                >::new_unchecked(self.compile)
                            };
                            let (ret0,) = callee
                                .call_async(store.as_context_mut(), (arg0, arg1))
                                .await?;
                            callee.post_return_async(store.as_context_mut()).await?;
                            Ok(ret0)
                        }
                    }
                }
            }
        }
    }
    const _: &str = "\r\npackage snippet:snippet@0.1.0;\r\n\r\nworld c {\r\n    import snippet:plugin/types@0.1.0;\r\n\r\n    export snippet:plugin/language@0.1.0;\r\n    export snippet:plugin/compiler@0.1.0;\r\n}";
    const _: &str = "\r\npackage snippet:plugin@0.1.0;\r\n\r\ninterface compiler {\r\n    use types.{error-type, mode, lang, cmd-result};\r\n\r\n    /// the PathBuf of binary\r\n    bin: func() -> result<list<u8>, error-type>;\r\n\r\n    support: func() -> lang;\r\n\r\n    resource compiler {\r\n        constructor();\r\n\r\n        /// Current arguments.\r\n        args: func() -> result<list<string>, error-type>;\r\n\r\n        /// Is the compiler in debug mode?\r\n        debug: func() -> result<bool, error-type>;\r\n\r\n        /// Current compile mode.\r\n        mode: func() -> result<mode, error-type>;\r\n\r\n        /// Set debug mode.\r\n        set-debug: func(debug: bool) -> result<_, error-type>;\r\n\r\n        /// Set the compile mode.\r\n        set-mode: func(mode: mode) -> result<_, error-type>;\r\n\r\n        /// Set the optimize level.\r\n        set-opt-level: func(level: u8) -> result<_, error-type>;\r\n\r\n        /// Set the language standard.\r\n        set-standard: func(std: string) -> result<_, error-type>;\r\n\r\n        /// Add a macro.\r\n        add-macro: func(macro: string, value: option<string>) -> result<_, error-type>;\r\n\r\n        /// Add include path.\r\n        add-include-path: func(path: string) -> result<_, error-type>;\r\n\r\n        /// Add library path.\r\n        add-library-path: func(path: string) -> result<_, error-type>;\r\n\r\n        /// Link a library.\r\n        link-library: func(library: string) -> result<_, error-type>;\r\n\r\n        /// Add an argument.\r\n        add-arg: func(arg: string) -> result<_, error-type>;\r\n\r\n        /// Append arguments.\r\n        add-args: func(args: list<string>) -> result<_, error-type>;\r\n\r\n        /// Compile the code\r\n        compile-code: func(source: list<string>, out: string) -> result<cmd-result, error-type>;\r\n\r\n        /// Compile the file\r\n        compile-file: func(path: string, out: string) -> result<cmd-result, error-type>;\r\n\r\n        /// Link the object into executable\r\n        link-object: func(objs: list<string>, out: string) -> result<cmd-result, error-type>;\r\n    }\r\n}";
    const _: &str = "\r\npackage snippet:plugin@0.1.0;\r\n\r\ninterface language {\r\n    use types.{error-type, mode, optset, lang, cmd-result};\r\n    use compiler.{compiler};\r\n\r\n    name: func() -> lang;\r\n    \r\n    initialize-optset: func() -> result<optset, error-type>;\r\n\r\n    fill-optset: func(optset: optset) -> result<_, error-type>;\r\n\r\n    compile: func(optset: optset, compiler: compiler) -> result<cmd-result, error-type>;\r\n}";
    const _: &str = "package snippet:plugin@0.1.0;\r\n\r\ninterface types {\r\n\r\n    enum lang {\r\n        c,\r\n\r\n        cxx,\r\n\r\n        rust,\r\n    }\r\n\r\n    enum mode {\r\n        /// Compile the code to object\r\n        compile,\r\n\r\n        /// Expand the macro or do preprocessing\r\n        expand,\r\n\r\n        /// Compile the code to assembly\r\n        assemble,\r\n\r\n        /// Compile the code and link the object to executable\r\n        link,\r\n    }\r\n\r\n    enum error-type {\r\n        /// Not support given mode\r\n        invalid-mode,\r\n\r\n        /// Not support given optimization level\r\n        invalid-opt-level,\r\n\r\n        /// Not support given language standard\r\n        invalid-standard,\r\n\r\n        /// Not found given binary\r\n        invalid-binary,\r\n\r\n        /// Invalid option string\r\n        invalid-optstr,\r\n\r\n        /// Can not get optset resource from resource table\r\n        invalid-optset-resource,\r\n\r\n        /// Can not get services resource from resource table\r\n        invalid-services-resource,\r\n\r\n        /// Failed read/write command stdin/stdout\r\n        command-io-failed,\r\n\r\n        /// Command need stdin for write\r\n        command-need-stdin,\r\n\r\n        /// Failed invoke command\r\n        command-invoke-failed,\r\n\r\n        /// Command spawn failed\r\n        command-spawn-failed,\r\n\r\n        /// Initialize optset failed\r\n        create-optset-failed,\r\n\r\n        /// Can not access value of optset\r\n        access-value-failed,\r\n    }\r\n\r\n    record cmd-result {\r\n        out: list<u8>,\r\n        err: list<u8>,\r\n        ret: s32,\r\n    }\r\n\r\n    resource optset {\r\n        constructor();\r\n\r\n        default: static func() -> result<optset, error-type>;\r\n\r\n        /// Add an option to the option set\r\n        add-opt: func(opt: string) -> result<u64, error-type>;\r\n\r\n        get-value-str: func(name: string) -> result<string, error-type>;\r\n    }\r\n\r\n    resource services {\r\n        constructor();\r\n\r\n        /// Is the compiler in debug mode?\r\n        debug: func() -> result<bool, error-type>;\r\n    \r\n        /// Current language.\r\n        lang: func() -> result<lang, error-type>;\r\n    \r\n        /// Current arguments.\r\n        args: func() -> result<list<string>, error-type>;\r\n    \r\n        /// Current compile mode.\r\n        mode: func() -> result<mode, error-type>;\r\n        \r\n        /// Set the language.\r\n        set-lang: func(language: lang) -> result<_, error-type>;\r\n    \r\n        /// Set debug mode.\r\n        set-debug: func(debug: bool) -> result<_, error-type>;\r\n    \r\n        /// Set the compile mode.\r\n        set-mode: func(mode: mode) -> result<_, error-type>;\r\n    \r\n        /// Add an argument.\r\n        add-arg: func(arg: string) -> result<_, error-type>;\r\n    \r\n        /// Append arguments.\r\n        add-args: func(args: list<string>) -> result<_, error-type>;\r\n\r\n        /// Find the executable binary.\r\n        find-bin: static func(bin: string) -> result<list<u8>, error-type>;\r\n\r\n        /// Invoke the command\r\n        invoke-cmd: static func(bin: list<u8>, args: list<string>, stdin: list<string>) -> result<cmd-result, error-type>;\r\n    }\r\n}";
    pub use exports::snippet::plugin::compiler;
    pub use exports::snippet::plugin::language;
    pub use snippet::plugin::types;
    use types::CmdResult;
    use types::ErrorType;
    use types::Lang;
    use types::Mode;
    pub struct OptSet {
        parser: Parser<'static, ASet, ASer>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for OptSet {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "OptSet",
                "parser",
                &&self.parser,
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for OptSet {
        #[inline]
        fn default() -> OptSet {
            OptSet {
                parser: ::core::default::Default::default(),
            }
        }
    }
    impl OptSet {
        pub fn new() -> Result<Self, cote::Error> {
            let mut parser = Parser::<'_, ASet, ASer>::default();
            parser.add_opt("-S=b: pass -S to compiler")?;
            parser.add_opt("-E=b: pass -E to compiler")?;
            parser.add_opt("-e=s: append code to generator")?;
            parser.add_opt("-r=b: ignore value of -e, read code from stdin")?;
            parser.add_opt("-end=s: set input code terminator")?;
            parser.add_opt("-o=s: set output name")?;
            parser.add_opt("-p=b: display code before display stdout")?;
            parser.add_opt("-l=s: pass -l to compiler, link given library")?;
            parser.add_opt("-L=s: pass -L to compiler, add link library searh path")?;
            parser.add_opt("-f=s: pass given flag -<f> to compiler")?;
            parser.add_opt("-flag=s: pass given flag --<flag> to compiler")?;
            parser.add_opt("-std=s: set language standard version")?;
            parser.add_opt("-c=s: select given compiler")?;
            parser.add_opt("-m=s: change the main function header")?;
            Ok(Self { parser })
        }
    }
    impl<T: WasiView> types::HostOptset for WasiImpl<T> {
        #[allow(
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn new<'life0, 'async_trait>(
            &'life0 mut self,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Resource<OptSet>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Resource<OptSet>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let __ret: Resource<OptSet> = {
                    __self.table().push(OptSet::default()).unwrap()
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn default<'life0, 'async_trait>(
            &'life0 mut self,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<Resource<OptSet>, ErrorType>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<Resource<OptSet>, ErrorType>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let __ret: Result<Resource<OptSet>, ErrorType> = {
                    let optset = OptSet::new()
                        .map_err(|_| ErrorType::CreateOptsetFailed)?;
                    __self
                        .table()
                        .push(optset)
                        .map_err(|_| ErrorType::InvalidOptsetResource)
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn add_opt<'life0, 'async_trait>(
            &'life0 mut self,
            self_: Resource<OptSet>,
            opt: String,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<u64, ErrorType>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<u64, ErrorType>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let self_ = self_;
                let opt = opt;
                let __ret: Result<u64, ErrorType> = {
                    let optset = __self
                        .table()
                        .get_mut(&self_)
                        .map_err(|_| ErrorType::InvalidOptsetResource)?;
                    Ok(
                        optset
                            .parser
                            .add_opt(opt)
                            .and_then(|v| v.run())
                            .map_err(|_| ErrorType::CommandInvokeFailed)?,
                    )
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn get_value_str<'life0, 'async_trait>(
            &'life0 mut self,
            self_: Resource<OptSet>,
            name: String,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<String, ErrorType>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<String, ErrorType>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let self_ = self_;
                let name = name;
                let __ret: Result<String, ErrorType> = {
                    let optset = __self
                        .table()
                        .get_mut(&self_)
                        .map_err(|_| ErrorType::InvalidOptsetResource)?;
                    optset
                        .parser
                        .find_val::<String>(name)
                        .map_err(|_| ErrorType::AccessValueFailed)
                        .cloned()
                };
                #[allow(unreachable_code)] __ret
            })
        }
        fn drop(&mut self, rep: Resource<OptSet>) -> wasmtime::Result<()> {
            self.table().delete(rep)?;
            Ok(())
        }
    }
    pub struct Services {
        debug: bool,
        lang: Lang,
        mode: Mode,
        args: Vec<String>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Services {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Services",
                "debug",
                &self.debug,
                "lang",
                &self.lang,
                "mode",
                &self.mode,
                "args",
                &&self.args,
            )
        }
    }
    impl Services {
        pub fn self_mut<T: WasiView>(
            wasi: &mut WasiImpl<T>,
            self_: Resource<Services>,
        ) -> Result<&mut Services, ErrorType> {
            wasi.table().get_mut(&self_).map_err(|_| ErrorType::InvalidServicesResource)
        }
        pub fn self_ref<T: WasiView>(
            wasi: &mut WasiImpl<T>,
            self_: Resource<Services>,
        ) -> Result<&Services, ErrorType> {
            wasi.table().get(&self_).map_err(|_| ErrorType::InvalidServicesResource)
        }
    }
    impl<T: WasiView> types::HostServices for WasiImpl<T> {
        #[allow(
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn new<'life0, 'async_trait>(
            &'life0 mut self,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Resource<Services>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Resource<Services>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let __ret: Resource<Services> = {
                    __self
                        .table()
                        .push(Services {
                            debug: false,
                            lang: Lang::C,
                            mode: Mode::Link,
                            args: ::alloc::vec::Vec::new(),
                        })
                        .unwrap()
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn debug<'life0, 'async_trait>(
            &'life0 mut self,
            self_: Resource<Services>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<bool, ErrorType>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<bool, ErrorType>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let self_ = self_;
                let __ret: Result<bool, ErrorType> = {
                    Services::self_ref(__self, self_).map(|v| v.debug)
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn lang<'life0, 'async_trait>(
            &'life0 mut self,
            self_: Resource<Services>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<Lang, ErrorType>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<Lang, ErrorType>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let self_ = self_;
                let __ret: Result<Lang, ErrorType> = {
                    Services::self_ref(__self, self_).map(|v| v.lang)
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn args<'life0, 'async_trait>(
            &'life0 mut self,
            self_: Resource<Services>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<Vec<String>, ErrorType>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<Vec<String>, ErrorType>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let self_ = self_;
                let __ret: Result<Vec<String>, ErrorType> = {
                    Services::self_ref(__self, self_).map(|v| v.args.clone())
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn mode<'life0, 'async_trait>(
            &'life0 mut self,
            self_: Resource<Services>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<Mode, ErrorType>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<Mode, ErrorType>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let self_ = self_;
                let __ret: Result<Mode, ErrorType> = {
                    Services::self_ref(__self, self_).map(|v| v.mode)
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn set_lang<'life0, 'async_trait>(
            &'life0 mut self,
            self_: Resource<Services>,
            language: Lang,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<(), ErrorType>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<(), ErrorType>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let self_ = self_;
                let language = language;
                let __ret: Result<(), ErrorType> = {
                    Services::self_mut(__self, self_)
                        .map(|v| {
                            v.lang = language;
                        })
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn set_debug<'life0, 'async_trait>(
            &'life0 mut self,
            self_: Resource<Services>,
            debug: bool,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<(), ErrorType>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<(), ErrorType>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let self_ = self_;
                let debug = debug;
                let __ret: Result<(), ErrorType> = {
                    Services::self_mut(__self, self_)
                        .map(|v| {
                            v.debug = debug;
                        })
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn set_mode<'life0, 'async_trait>(
            &'life0 mut self,
            self_: Resource<Services>,
            mode: Mode,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<(), ErrorType>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<(), ErrorType>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let self_ = self_;
                let mode = mode;
                let __ret: Result<(), ErrorType> = {
                    Services::self_mut(__self, self_)
                        .map(|v| {
                            v.mode = mode;
                        })
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn add_arg<'life0, 'async_trait>(
            &'life0 mut self,
            self_: Resource<Services>,
            arg: String,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<(), ErrorType>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<(), ErrorType>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let self_ = self_;
                let arg = arg;
                let __ret: Result<(), ErrorType> = {
                    Services::self_mut(__self, self_)
                        .map(|v| {
                            v.args.push(arg);
                        })
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn add_args<'life0, 'async_trait>(
            &'life0 mut self,
            self_: Resource<Services>,
            args: Vec<String>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<(), ErrorType>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<(), ErrorType>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let self_ = self_;
                let args = args;
                let __ret: Result<(), ErrorType> = {
                    Services::self_mut(__self, self_)
                        .map(|v| {
                            v.args.extend(args);
                        })
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn find_bin<'life0, 'async_trait>(
            &'life0 mut self,
            bin: String,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<Vec<u8>, ErrorType>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<Vec<u8>, ErrorType>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let bin = bin;
                let __ret: Result<Vec<u8>, ErrorType> = {
                    which::which(bin)
                        .map_err(|_| ErrorType::InvalidBinary)
                        .map(|v| v.as_os_str().as_encoded_bytes().to_vec())
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn invoke_cmd<'life0, 'async_trait>(
            &'life0 mut self,
            bin: Vec<u8>,
            args: Vec<String>,
            stdin_lines: Vec<String>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<CmdResult, ErrorType>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<CmdResult, ErrorType>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let mut __self = self;
                let bin = bin;
                let args = args;
                let stdin_lines = stdin_lines;
                let __ret: Result<CmdResult, ErrorType> = {
                    let bin = Path::new(unsafe {
                        OsStr::from_encoded_bytes_unchecked(&bin)
                    });
                    let mut cmd = Command::new(bin);
                    cmd.args(args);
                    let mut child = cmd
                        .spawn()
                        .map_err(|_| ErrorType::CommandSpawnFailed)?;
                    match child.stdin.as_mut() {
                        Some(stdin) => {
                            for line in stdin_lines {
                                stdin
                                    .write_all(
                                        {
                                            let res = ::alloc::fmt::format(format_args!("{0}\n", line));
                                            res
                                        }
                                            .as_bytes(),
                                    )
                                    .await
                                    .map_err(|_| ErrorType::CommandIoFailed)?;
                            }
                        }
                        None => {
                            if stdin_lines.is_empty() {
                                return Err(ErrorType::CommandNeedStdin);
                            }
                        }
                    }
                    child
                        .wait_with_output()
                        .await
                        .map_err(|_| ErrorType::CommandInvokeFailed)
                        .map(|v| CmdResult {
                            err: v.stderr,
                            out: v.stdout,
                            ret: v.status.code().unwrap_or_default(),
                        })
                };
                #[allow(unreachable_code)] __ret
            })
        }
        fn drop(&mut self, rep: Resource<Services>) -> wasmtime::Result<()> {
            self.table().delete(rep)?;
            Ok(())
        }
    }
    impl<T: WasiView> types::Host for WasiImpl<T> {}
}
pub mod link {
    use std::path::Path;
    use wac_graph::{types::Package, CompositionGraph, EncodeOptions};
    pub fn link_component(a: &Path, b: &Path) -> wasmtime::Result<Vec<u8>> {
        let mut graph = CompositionGraph::new();
        let compiler = Package::from_file("compiler_c", None, a, graph.types_mut())?;
        let compiler = graph.register_package(compiler)?;
        let language = Package::from_file("language_c", None, b, graph.types_mut())?;
        let language = graph.register_package(language)?;
        let compiler_ins = graph.instantiate(compiler);
        let language_ins = graph.instantiate(language);
        let comp_comp = graph
            .alias_instance_export(compiler_ins, "snippet:plugin/compiler@0.1.0")?;
        graph
            .set_instantiation_argument(
                language_ins,
                "snippet:plugin/compiler@0.1.0",
                comp_comp,
            )?;
        let lang_lang = graph
            .alias_instance_export(language_ins, "snippet:plugin/language@0.1.0")?;
        graph.export(lang_lang, "snippet:plugin/language@0.1.0")?;
        graph.export(comp_comp, "snippet:plugin/compiler@0.1.0")?;
        Ok(graph.encode(EncodeOptions::default())?)
    }
}
use cote::prelude::*;
use host::Root;
use link::link_component;
use std::path::PathBuf;
use wasmtime::component::*;
use wasmtime::Config;
use wasmtime::Store;
use wasmtime_wasi::ResourceTable;
use wasmtime_wasi::WasiCtx;
use wasmtime_wasi::WasiCtxBuilder;
use wasmtime_wasi::{WasiImpl, WasiView};
pub struct State {
    ctx: WasiCtx,
    table: ResourceTable,
}
impl WasiView for State {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}
fn type_annotate<T: WasiView, F>(val: F) -> F
where
    F: Fn(&mut T) -> WasiImpl<&mut T>,
{
    val
}
pub struct C;
#[automatically_derived]
impl ::core::fmt::Debug for C {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "C")
    }
}
///Automatic generated by cote-derive for [`C`].
impl<'inv, Set, Ser> cote::IntoParserDerive<'inv, Set, Ser> for C
where
    Set: cote::prelude::Set + cote::prelude::OptParser + cote::prelude::OptValidator
        + cote::prelude::SetValueFindExt + Default + 'inv,
    Ser: cote::prelude::ServicesValExt + Default + 'inv,
    cote::prelude::SetCfg<Set>: cote::prelude::ConfigValue + Default,
    <Set as cote::prelude::OptParser>::Output: cote::prelude::Information,
{
    fn update(parser: &mut cote::prelude::Parser<'inv, Set, Ser>) -> cote::Result<()> {
        type InferedOptVal<T> = <T as cote::prelude::Infer>::Val;
        let set = parser.optset_mut();
        let ctor_name = cote::prelude::ctor_default_name();
        Ok(())
    }
}
///Automatic generated by cote-derive for [`C`].
impl<'set, Set> cote::ExtractFromSetDerive<'set, Set> for C
where
    Set: cote::prelude::SetValueFindExt,
    cote::prelude::SetCfg<Set>: cote::prelude::ConfigValue + Default,
{
    fn try_extract(set: &'set mut Set) -> cote::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}
///Automatic generated by cote-derive for [`C`].
impl C {
    #[doc(hidden)]
    pub fn new_help_context() -> cote::prelude::HelpContext {
        cote::prelude::HelpContext::default()
            .with_name(String::from("snippet"))
            .with_head(String::from(""))
            .with_foot({
                let res = ::alloc::fmt::format(
                    format_args!("Create by {0} v{1}", "araraloren", "0.1.0"),
                );
                res
            })
            .with_width(40usize)
            .with_usagew(10usize)
    }
    #[doc(hidden)]
    pub fn sync_rctx<'a, Set, Ret>(
        rctx: &'a mut cote::prelude::RunningCtx,
        ret: &cote::Result<Ret>,
        set: &Set,
        sub_parser: bool,
    ) -> cote::Result<&'a mut cote::prelude::RunningCtx>
    where
        Set: cote::prelude::SetValueFindExt,
        Ret: cote::prelude::Status,
        cote::prelude::SetCfg<Set>: cote::prelude::ConfigValue + Default,
    {
        Ok(rctx)
    }
    pub fn into_parser<'inv>() -> cote::Result<
        cote::prelude::Parser<'inv, cote::prelude::ASet, cote::prelude::ASer>,
    > {
        Self::into_parser_with::<cote::prelude::ASet, cote::prelude::ASer>()
    }
    pub fn into_parser_with<'inv, Set, Ser>() -> cote::Result<
        cote::prelude::Parser<'inv, Set, Ser>,
    >
    where
        Ser: cote::prelude::ServicesValExt + Default + 'inv,
        cote::prelude::SetCfg<Set>: cote::prelude::ConfigValue + Default,
        <Set as cote::prelude::OptParser>::Output: cote::prelude::Information,
        Set: cote::prelude::Set + cote::prelude::OptParser + cote::prelude::OptValidator
            + cote::prelude::SetValueFindExt + Default + 'inv,
    {
        let mut parser = <Self as cote::IntoParserDerive<
            'inv,
            Set,
            Ser,
        >>::into_parser()?;
        Ok(parser.with_name(String::from("snippet")))
    }
    pub fn into_policy<'inv>() -> cote::prelude::FwdPolicy<
        'inv,
        cote::prelude::ASet,
        cote::prelude::ASer,
    > {
        Self::into_policy_with()
    }
    pub fn into_policy_with<'inv, Set, Ser>() -> cote::prelude::FwdPolicy<
        'inv,
        Set,
        Ser,
    > {
        let mut policy: cote::prelude::FwdPolicy<'inv, Set, Ser> = Default::default();
        Self::apply_policy_settings(&mut policy);
        policy
    }
    pub fn apply_policy_settings(policy: &mut impl cote::prelude::PolicySettings) {
        let style_manager = policy.style_manager_mut();
    }
    pub fn parse_args_with<'inv, Set, Ser, P>(
        args: cote::prelude::Args,
        policy: &mut P,
    ) -> cote::Result<cote::prelude::CoteRes<&mut P, P>>
    where
        P::Error: Into<cote::Error>,
        P::Ret: cote::prelude::Status,
        Ser: cote::prelude::ServicesValExt + Default + 'inv,
        cote::prelude::SetCfg<Set>: cote::prelude::ConfigValue + Default,
        <Set as cote::prelude::OptParser>::Output: cote::prelude::Information,
        Set: cote::prelude::Set + cote::prelude::OptParser + cote::prelude::OptValidator
            + cote::prelude::SetValueFindExt + Default + 'inv,
        P: cote::prelude::Policy<
                Set = cote::prelude::Parser<'inv, Set, Ser>,
                Ser = Ser,
                Inv<
                    'inv,
                > = cote::prelude::Invoker<
                    'inv,
                    cote::prelude::Parser<'inv, Set, Ser>,
                    Ser,
                >,
            > + cote::prelude::APolicyExt<P> + cote::prelude::PolicySettings + Default,
    {
        let mut parser = Self::into_parser_with::<'inv, Set, Ser>()?;
        let mut rctx = cote::prelude::RunningCtx::default();
        rctx.add_name(String::from("snippet"));
        parser.set_rctx(rctx);
        let args = cote::prelude::ARef::new(args);
        let ret = cote::prelude::PolicyParser::parse_policy(&mut parser, args, policy);
        let mut rctx = parser.take_rctx()?;
        if !rctx.display_help() {
            Self::sync_rctx::<Set, _>(&mut rctx, &ret, parser.optset(), false)?;
            if rctx.display_help() {
                rctx.set_help_context(Self::new_help_context());
            }
        }
        parser.set_rctx(rctx);
        let mut rctx = parser.rctx()?;
        if rctx.display_help() {
            let names = rctx.names().iter().map(|v| v.as_str()).collect::<Vec<&str>>();
            let help_context = rctx.help_context().unwrap();
            let exit = rctx.exit();
            parser.display_sub_help(names, &help_context)?;
            if exit {
                std::process::exit(0);
            }
        }
        Ok(cote::prelude::CoteRes {
            ret: ret?,
            parser,
            policy,
        })
    }
    pub fn parse_args<'inv>(
        args: cote::prelude::Args,
    ) -> cote::Result<
        cote::prelude::CoteRes<
            cote::prelude::FwdPolicy<'inv, cote::prelude::ASet, cote::prelude::ASer>,
            cote::prelude::FwdPolicy<'inv, cote::prelude::ASet, cote::prelude::ASer>,
        >,
    > {
        let mut policy = Self::into_policy();
        let cote::prelude::CoteRes { ret, parser, .. } = Self::parse_args_with(
            args,
            &mut policy,
        )?;
        Ok(cote::prelude::CoteRes {
            ret,
            parser,
            policy,
        })
    }
    pub fn parse(args: cote::prelude::Args) -> cote::Result<Self> {
        let cote::prelude::CoteRes { mut ret, mut parser, .. } = Self::parse_args(args)?;
        if let Some(mut error) = ret.take_failure() {
            let mut rctx = parser.take_rctx()?;
            if let Some(chain_error) = rctx.chain_error() {
                error = error.cause_by(chain_error);
            }
            let mut failed_info = rctx.take_failed_info();
            let (command, ret) = failed_info
                .last_mut()
                .map(|v| (Some(v.name.as_str()), &mut v.retval))
                .unwrap_or((None, &mut ret));
            let e = {
                let ctx = ret.take_ctx();
                let args = ctx
                    .orig_args()[1..]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                let inner_ctx = ctx.inner_ctx().ok();
                let failed_msg = if let Some(command) = command {
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("Parsing command `{0}`", command),
                        );
                        res
                    }
                } else {
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("Parsing arguments `{0}`", args),
                        );
                        res
                    }
                };
                let inner_ctx = if let Some(inner_ctx) = inner_ctx {
                    {
                        let res = ::alloc::fmt::format(format_args!("{0}", inner_ctx));
                        res
                    }
                } else {
                    "None".to_owned()
                };
                ::aopt::Error::raise_failure({
                        let res = ::alloc::fmt::format(
                            format_args!("{0} failed: {1}", failed_msg, inner_ctx),
                        );
                        res
                    })
                    .cause_by(error)
            };
            Err(e)
        } else {
            <Self as cote::ExtractFromSetDerive<
                cote::prelude::ASet,
            >>::try_extract(parser.optset_mut())
        }
    }
    pub fn parse_env_args_with<'inv, Set, Ser, P>(
        policy: &mut P,
    ) -> cote::Result<cote::prelude::CoteRes<&mut P, P>>
    where
        P::Error: Into<cote::Error>,
        P::Ret: cote::prelude::Status,
        Ser: cote::prelude::ServicesValExt + Default + 'inv,
        cote::prelude::SetCfg<Set>: cote::prelude::ConfigValue + Default,
        <Set as cote::prelude::OptParser>::Output: cote::prelude::Information,
        Set: cote::prelude::Set + cote::prelude::OptParser + cote::prelude::OptValidator
            + cote::prelude::SetValueFindExt + Default + 'inv,
        P: cote::prelude::Policy<
                Set = cote::prelude::Parser<'inv, Set, Ser>,
                Ser = Ser,
                Inv<
                    'inv,
                > = cote::prelude::Invoker<
                    'inv,
                    cote::prelude::Parser<'inv, Set, Ser>,
                    Ser,
                >,
            > + cote::prelude::APolicyExt<P> + cote::prelude::PolicySettings + Default,
    {
        Self::parse_args_with(cote::prelude::Args::from_env(), policy)
    }
    pub fn parse_env_args<'inv>() -> cote::Result<
        cote::prelude::CoteRes<
            cote::prelude::FwdPolicy<'inv, cote::prelude::ASet, cote::prelude::ASer>,
            cote::prelude::FwdPolicy<'inv, cote::prelude::ASet, cote::prelude::ASer>,
        >,
    > {
        Self::parse_args(cote::prelude::Args::from_env())
    }
    pub fn parse_env() -> cote::Result<Self> {
        Self::parse(cote::prelude::Args::from_env())
    }
}
pub struct Cpp;
#[automatically_derived]
impl ::core::fmt::Debug for Cpp {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "Cpp")
    }
}
///Automatic generated by cote-derive for [`Cpp`].
impl<'inv, Set, Ser> cote::IntoParserDerive<'inv, Set, Ser> for Cpp
where
    Set: cote::prelude::Set + cote::prelude::OptParser + cote::prelude::OptValidator
        + cote::prelude::SetValueFindExt + Default + 'inv,
    Ser: cote::prelude::ServicesValExt + Default + 'inv,
    cote::prelude::SetCfg<Set>: cote::prelude::ConfigValue + Default,
    <Set as cote::prelude::OptParser>::Output: cote::prelude::Information,
{
    fn update(parser: &mut cote::prelude::Parser<'inv, Set, Ser>) -> cote::Result<()> {
        type InferedOptVal<T> = <T as cote::prelude::Infer>::Val;
        let set = parser.optset_mut();
        let ctor_name = cote::prelude::ctor_default_name();
        Ok(())
    }
}
///Automatic generated by cote-derive for [`Cpp`].
impl<'set, Set> cote::ExtractFromSetDerive<'set, Set> for Cpp
where
    Set: cote::prelude::SetValueFindExt,
    cote::prelude::SetCfg<Set>: cote::prelude::ConfigValue + Default,
{
    fn try_extract(set: &'set mut Set) -> cote::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}
///Automatic generated by cote-derive for [`Cpp`].
impl Cpp {
    #[doc(hidden)]
    pub fn new_help_context() -> cote::prelude::HelpContext {
        cote::prelude::HelpContext::default()
            .with_name(String::from("snippet"))
            .with_head(String::from(""))
            .with_foot({
                let res = ::alloc::fmt::format(
                    format_args!("Create by {0} v{1}", "araraloren", "0.1.0"),
                );
                res
            })
            .with_width(40usize)
            .with_usagew(10usize)
    }
    #[doc(hidden)]
    pub fn sync_rctx<'a, Set, Ret>(
        rctx: &'a mut cote::prelude::RunningCtx,
        ret: &cote::Result<Ret>,
        set: &Set,
        sub_parser: bool,
    ) -> cote::Result<&'a mut cote::prelude::RunningCtx>
    where
        Set: cote::prelude::SetValueFindExt,
        Ret: cote::prelude::Status,
        cote::prelude::SetCfg<Set>: cote::prelude::ConfigValue + Default,
    {
        Ok(rctx)
    }
    pub fn into_parser<'inv>() -> cote::Result<
        cote::prelude::Parser<'inv, cote::prelude::ASet, cote::prelude::ASer>,
    > {
        Self::into_parser_with::<cote::prelude::ASet, cote::prelude::ASer>()
    }
    pub fn into_parser_with<'inv, Set, Ser>() -> cote::Result<
        cote::prelude::Parser<'inv, Set, Ser>,
    >
    where
        Ser: cote::prelude::ServicesValExt + Default + 'inv,
        cote::prelude::SetCfg<Set>: cote::prelude::ConfigValue + Default,
        <Set as cote::prelude::OptParser>::Output: cote::prelude::Information,
        Set: cote::prelude::Set + cote::prelude::OptParser + cote::prelude::OptValidator
            + cote::prelude::SetValueFindExt + Default + 'inv,
    {
        let mut parser = <Self as cote::IntoParserDerive<
            'inv,
            Set,
            Ser,
        >>::into_parser()?;
        Ok(parser.with_name(String::from("snippet")))
    }
    pub fn into_policy<'inv>() -> cote::prelude::FwdPolicy<
        'inv,
        cote::prelude::ASet,
        cote::prelude::ASer,
    > {
        Self::into_policy_with()
    }
    pub fn into_policy_with<'inv, Set, Ser>() -> cote::prelude::FwdPolicy<
        'inv,
        Set,
        Ser,
    > {
        let mut policy: cote::prelude::FwdPolicy<'inv, Set, Ser> = Default::default();
        Self::apply_policy_settings(&mut policy);
        policy
    }
    pub fn apply_policy_settings(policy: &mut impl cote::prelude::PolicySettings) {
        let style_manager = policy.style_manager_mut();
    }
    pub fn parse_args_with<'inv, Set, Ser, P>(
        args: cote::prelude::Args,
        policy: &mut P,
    ) -> cote::Result<cote::prelude::CoteRes<&mut P, P>>
    where
        P::Error: Into<cote::Error>,
        P::Ret: cote::prelude::Status,
        Ser: cote::prelude::ServicesValExt + Default + 'inv,
        cote::prelude::SetCfg<Set>: cote::prelude::ConfigValue + Default,
        <Set as cote::prelude::OptParser>::Output: cote::prelude::Information,
        Set: cote::prelude::Set + cote::prelude::OptParser + cote::prelude::OptValidator
            + cote::prelude::SetValueFindExt + Default + 'inv,
        P: cote::prelude::Policy<
                Set = cote::prelude::Parser<'inv, Set, Ser>,
                Ser = Ser,
                Inv<
                    'inv,
                > = cote::prelude::Invoker<
                    'inv,
                    cote::prelude::Parser<'inv, Set, Ser>,
                    Ser,
                >,
            > + cote::prelude::APolicyExt<P> + cote::prelude::PolicySettings + Default,
    {
        let mut parser = Self::into_parser_with::<'inv, Set, Ser>()?;
        let mut rctx = cote::prelude::RunningCtx::default();
        rctx.add_name(String::from("snippet"));
        parser.set_rctx(rctx);
        let args = cote::prelude::ARef::new(args);
        let ret = cote::prelude::PolicyParser::parse_policy(&mut parser, args, policy);
        let mut rctx = parser.take_rctx()?;
        if !rctx.display_help() {
            Self::sync_rctx::<Set, _>(&mut rctx, &ret, parser.optset(), false)?;
            if rctx.display_help() {
                rctx.set_help_context(Self::new_help_context());
            }
        }
        parser.set_rctx(rctx);
        let mut rctx = parser.rctx()?;
        if rctx.display_help() {
            let names = rctx.names().iter().map(|v| v.as_str()).collect::<Vec<&str>>();
            let help_context = rctx.help_context().unwrap();
            let exit = rctx.exit();
            parser.display_sub_help(names, &help_context)?;
            if exit {
                std::process::exit(0);
            }
        }
        Ok(cote::prelude::CoteRes {
            ret: ret?,
            parser,
            policy,
        })
    }
    pub fn parse_args<'inv>(
        args: cote::prelude::Args,
    ) -> cote::Result<
        cote::prelude::CoteRes<
            cote::prelude::FwdPolicy<'inv, cote::prelude::ASet, cote::prelude::ASer>,
            cote::prelude::FwdPolicy<'inv, cote::prelude::ASet, cote::prelude::ASer>,
        >,
    > {
        let mut policy = Self::into_policy();
        let cote::prelude::CoteRes { ret, parser, .. } = Self::parse_args_with(
            args,
            &mut policy,
        )?;
        Ok(cote::prelude::CoteRes {
            ret,
            parser,
            policy,
        })
    }
    pub fn parse(args: cote::prelude::Args) -> cote::Result<Self> {
        let cote::prelude::CoteRes { mut ret, mut parser, .. } = Self::parse_args(args)?;
        if let Some(mut error) = ret.take_failure() {
            let mut rctx = parser.take_rctx()?;
            if let Some(chain_error) = rctx.chain_error() {
                error = error.cause_by(chain_error);
            }
            let mut failed_info = rctx.take_failed_info();
            let (command, ret) = failed_info
                .last_mut()
                .map(|v| (Some(v.name.as_str()), &mut v.retval))
                .unwrap_or((None, &mut ret));
            let e = {
                let ctx = ret.take_ctx();
                let args = ctx
                    .orig_args()[1..]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                let inner_ctx = ctx.inner_ctx().ok();
                let failed_msg = if let Some(command) = command {
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("Parsing command `{0}`", command),
                        );
                        res
                    }
                } else {
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("Parsing arguments `{0}`", args),
                        );
                        res
                    }
                };
                let inner_ctx = if let Some(inner_ctx) = inner_ctx {
                    {
                        let res = ::alloc::fmt::format(format_args!("{0}", inner_ctx));
                        res
                    }
                } else {
                    "None".to_owned()
                };
                ::aopt::Error::raise_failure({
                        let res = ::alloc::fmt::format(
                            format_args!("{0} failed: {1}", failed_msg, inner_ctx),
                        );
                        res
                    })
                    .cause_by(error)
            };
            Err(e)
        } else {
            <Self as cote::ExtractFromSetDerive<
                cote::prelude::ASet,
            >>::try_extract(parser.optset_mut())
        }
    }
    pub fn parse_env_args_with<'inv, Set, Ser, P>(
        policy: &mut P,
    ) -> cote::Result<cote::prelude::CoteRes<&mut P, P>>
    where
        P::Error: Into<cote::Error>,
        P::Ret: cote::prelude::Status,
        Ser: cote::prelude::ServicesValExt + Default + 'inv,
        cote::prelude::SetCfg<Set>: cote::prelude::ConfigValue + Default,
        <Set as cote::prelude::OptParser>::Output: cote::prelude::Information,
        Set: cote::prelude::Set + cote::prelude::OptParser + cote::prelude::OptValidator
            + cote::prelude::SetValueFindExt + Default + 'inv,
        P: cote::prelude::Policy<
                Set = cote::prelude::Parser<'inv, Set, Ser>,
                Ser = Ser,
                Inv<
                    'inv,
                > = cote::prelude::Invoker<
                    'inv,
                    cote::prelude::Parser<'inv, Set, Ser>,
                    Ser,
                >,
            > + cote::prelude::APolicyExt<P> + cote::prelude::PolicySettings + Default,
    {
        Self::parse_args_with(cote::prelude::Args::from_env(), policy)
    }
    pub fn parse_env_args<'inv>() -> cote::Result<
        cote::prelude::CoteRes<
            cote::prelude::FwdPolicy<'inv, cote::prelude::ASet, cote::prelude::ASer>,
            cote::prelude::FwdPolicy<'inv, cote::prelude::ASet, cote::prelude::ASer>,
        >,
    > {
        Self::parse_args(cote::prelude::Args::from_env())
    }
    pub fn parse_env() -> cote::Result<Self> {
        Self::parse(cote::prelude::Args::from_env())
    }
}
pub struct Rust;
#[automatically_derived]
impl ::core::fmt::Debug for Rust {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "Rust")
    }
}
///Automatic generated by cote-derive for [`Rust`].
impl<'inv, Set, Ser> cote::IntoParserDerive<'inv, Set, Ser> for Rust
where
    Set: cote::prelude::Set + cote::prelude::OptParser + cote::prelude::OptValidator
        + cote::prelude::SetValueFindExt + Default + 'inv,
    Ser: cote::prelude::ServicesValExt + Default + 'inv,
    cote::prelude::SetCfg<Set>: cote::prelude::ConfigValue + Default,
    <Set as cote::prelude::OptParser>::Output: cote::prelude::Information,
{
    fn update(parser: &mut cote::prelude::Parser<'inv, Set, Ser>) -> cote::Result<()> {
        type InferedOptVal<T> = <T as cote::prelude::Infer>::Val;
        let set = parser.optset_mut();
        let ctor_name = cote::prelude::ctor_default_name();
        Ok(())
    }
}
///Automatic generated by cote-derive for [`Rust`].
impl<'set, Set> cote::ExtractFromSetDerive<'set, Set> for Rust
where
    Set: cote::prelude::SetValueFindExt,
    cote::prelude::SetCfg<Set>: cote::prelude::ConfigValue + Default,
{
    fn try_extract(set: &'set mut Set) -> cote::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}
///Automatic generated by cote-derive for [`Rust`].
impl Rust {
    #[doc(hidden)]
    pub fn new_help_context() -> cote::prelude::HelpContext {
        cote::prelude::HelpContext::default()
            .with_name(String::from("snippet"))
            .with_head(String::from(""))
            .with_foot({
                let res = ::alloc::fmt::format(
                    format_args!("Create by {0} v{1}", "araraloren", "0.1.0"),
                );
                res
            })
            .with_width(40usize)
            .with_usagew(10usize)
    }
    #[doc(hidden)]
    pub fn sync_rctx<'a, Set, Ret>(
        rctx: &'a mut cote::prelude::RunningCtx,
        ret: &cote::Result<Ret>,
        set: &Set,
        sub_parser: bool,
    ) -> cote::Result<&'a mut cote::prelude::RunningCtx>
    where
        Set: cote::prelude::SetValueFindExt,
        Ret: cote::prelude::Status,
        cote::prelude::SetCfg<Set>: cote::prelude::ConfigValue + Default,
    {
        Ok(rctx)
    }
    pub fn into_parser<'inv>() -> cote::Result<
        cote::prelude::Parser<'inv, cote::prelude::ASet, cote::prelude::ASer>,
    > {
        Self::into_parser_with::<cote::prelude::ASet, cote::prelude::ASer>()
    }
    pub fn into_parser_with<'inv, Set, Ser>() -> cote::Result<
        cote::prelude::Parser<'inv, Set, Ser>,
    >
    where
        Ser: cote::prelude::ServicesValExt + Default + 'inv,
        cote::prelude::SetCfg<Set>: cote::prelude::ConfigValue + Default,
        <Set as cote::prelude::OptParser>::Output: cote::prelude::Information,
        Set: cote::prelude::Set + cote::prelude::OptParser + cote::prelude::OptValidator
            + cote::prelude::SetValueFindExt + Default + 'inv,
    {
        let mut parser = <Self as cote::IntoParserDerive<
            'inv,
            Set,
            Ser,
        >>::into_parser()?;
        Ok(parser.with_name(String::from("snippet")))
    }
    pub fn into_policy<'inv>() -> cote::prelude::FwdPolicy<
        'inv,
        cote::prelude::ASet,
        cote::prelude::ASer,
    > {
        Self::into_policy_with()
    }
    pub fn into_policy_with<'inv, Set, Ser>() -> cote::prelude::FwdPolicy<
        'inv,
        Set,
        Ser,
    > {
        let mut policy: cote::prelude::FwdPolicy<'inv, Set, Ser> = Default::default();
        Self::apply_policy_settings(&mut policy);
        policy
    }
    pub fn apply_policy_settings(policy: &mut impl cote::prelude::PolicySettings) {
        let style_manager = policy.style_manager_mut();
    }
    pub fn parse_args_with<'inv, Set, Ser, P>(
        args: cote::prelude::Args,
        policy: &mut P,
    ) -> cote::Result<cote::prelude::CoteRes<&mut P, P>>
    where
        P::Error: Into<cote::Error>,
        P::Ret: cote::prelude::Status,
        Ser: cote::prelude::ServicesValExt + Default + 'inv,
        cote::prelude::SetCfg<Set>: cote::prelude::ConfigValue + Default,
        <Set as cote::prelude::OptParser>::Output: cote::prelude::Information,
        Set: cote::prelude::Set + cote::prelude::OptParser + cote::prelude::OptValidator
            + cote::prelude::SetValueFindExt + Default + 'inv,
        P: cote::prelude::Policy<
                Set = cote::prelude::Parser<'inv, Set, Ser>,
                Ser = Ser,
                Inv<
                    'inv,
                > = cote::prelude::Invoker<
                    'inv,
                    cote::prelude::Parser<'inv, Set, Ser>,
                    Ser,
                >,
            > + cote::prelude::APolicyExt<P> + cote::prelude::PolicySettings + Default,
    {
        let mut parser = Self::into_parser_with::<'inv, Set, Ser>()?;
        let mut rctx = cote::prelude::RunningCtx::default();
        rctx.add_name(String::from("snippet"));
        parser.set_rctx(rctx);
        let args = cote::prelude::ARef::new(args);
        let ret = cote::prelude::PolicyParser::parse_policy(&mut parser, args, policy);
        let mut rctx = parser.take_rctx()?;
        if !rctx.display_help() {
            Self::sync_rctx::<Set, _>(&mut rctx, &ret, parser.optset(), false)?;
            if rctx.display_help() {
                rctx.set_help_context(Self::new_help_context());
            }
        }
        parser.set_rctx(rctx);
        let mut rctx = parser.rctx()?;
        if rctx.display_help() {
            let names = rctx.names().iter().map(|v| v.as_str()).collect::<Vec<&str>>();
            let help_context = rctx.help_context().unwrap();
            let exit = rctx.exit();
            parser.display_sub_help(names, &help_context)?;
            if exit {
                std::process::exit(0);
            }
        }
        Ok(cote::prelude::CoteRes {
            ret: ret?,
            parser,
            policy,
        })
    }
    pub fn parse_args<'inv>(
        args: cote::prelude::Args,
    ) -> cote::Result<
        cote::prelude::CoteRes<
            cote::prelude::FwdPolicy<'inv, cote::prelude::ASet, cote::prelude::ASer>,
            cote::prelude::FwdPolicy<'inv, cote::prelude::ASet, cote::prelude::ASer>,
        >,
    > {
        let mut policy = Self::into_policy();
        let cote::prelude::CoteRes { ret, parser, .. } = Self::parse_args_with(
            args,
            &mut policy,
        )?;
        Ok(cote::prelude::CoteRes {
            ret,
            parser,
            policy,
        })
    }
    pub fn parse(args: cote::prelude::Args) -> cote::Result<Self> {
        let cote::prelude::CoteRes { mut ret, mut parser, .. } = Self::parse_args(args)?;
        if let Some(mut error) = ret.take_failure() {
            let mut rctx = parser.take_rctx()?;
            if let Some(chain_error) = rctx.chain_error() {
                error = error.cause_by(chain_error);
            }
            let mut failed_info = rctx.take_failed_info();
            let (command, ret) = failed_info
                .last_mut()
                .map(|v| (Some(v.name.as_str()), &mut v.retval))
                .unwrap_or((None, &mut ret));
            let e = {
                let ctx = ret.take_ctx();
                let args = ctx
                    .orig_args()[1..]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                let inner_ctx = ctx.inner_ctx().ok();
                let failed_msg = if let Some(command) = command {
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("Parsing command `{0}`", command),
                        );
                        res
                    }
                } else {
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("Parsing arguments `{0}`", args),
                        );
                        res
                    }
                };
                let inner_ctx = if let Some(inner_ctx) = inner_ctx {
                    {
                        let res = ::alloc::fmt::format(format_args!("{0}", inner_ctx));
                        res
                    }
                } else {
                    "None".to_owned()
                };
                ::aopt::Error::raise_failure({
                        let res = ::alloc::fmt::format(
                            format_args!("{0} failed: {1}", failed_msg, inner_ctx),
                        );
                        res
                    })
                    .cause_by(error)
            };
            Err(e)
        } else {
            <Self as cote::ExtractFromSetDerive<
                cote::prelude::ASet,
            >>::try_extract(parser.optset_mut())
        }
    }
    pub fn parse_env_args_with<'inv, Set, Ser, P>(
        policy: &mut P,
    ) -> cote::Result<cote::prelude::CoteRes<&mut P, P>>
    where
        P::Error: Into<cote::Error>,
        P::Ret: cote::prelude::Status,
        Ser: cote::prelude::ServicesValExt + Default + 'inv,
        cote::prelude::SetCfg<Set>: cote::prelude::ConfigValue + Default,
        <Set as cote::prelude::OptParser>::Output: cote::prelude::Information,
        Set: cote::prelude::Set + cote::prelude::OptParser + cote::prelude::OptValidator
            + cote::prelude::SetValueFindExt + Default + 'inv,
        P: cote::prelude::Policy<
                Set = cote::prelude::Parser<'inv, Set, Ser>,
                Ser = Ser,
                Inv<
                    'inv,
                > = cote::prelude::Invoker<
                    'inv,
                    cote::prelude::Parser<'inv, Set, Ser>,
                    Ser,
                >,
            > + cote::prelude::APolicyExt<P> + cote::prelude::PolicySettings + Default,
    {
        Self::parse_args_with(cote::prelude::Args::from_env(), policy)
    }
    pub fn parse_env_args<'inv>() -> cote::Result<
        cote::prelude::CoteRes<
            cote::prelude::FwdPolicy<'inv, cote::prelude::ASet, cote::prelude::ASer>,
            cote::prelude::FwdPolicy<'inv, cote::prelude::ASet, cote::prelude::ASer>,
        >,
    > {
        Self::parse_args(cote::prelude::Args::from_env())
    }
    pub fn parse_env() -> cote::Result<Self> {
        Self::parse(cote::prelude::Args::from_env())
    }
}
#[cote(policy = pre, aborthelp)]
pub struct Snippet {
    #[sub()]
    c: Option<C>,
    #[sub(alias = "cxx", alias = "cc")]
    cpp: Option<Cpp>,
    #[sub(alias = "rs")]
    rust: Option<Rust>,
}
#[automatically_derived]
impl ::core::fmt::Debug for Snippet {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "Snippet",
            "c",
            &self.c,
            "cpp",
            &self.cpp,
            "rust",
            &&self.rust,
        )
    }
}
///Automatic generated by cote-derive for [`Snippet`].
impl<'inv, Set, Ser> cote::IntoParserDerive<'inv, Set, Ser> for Snippet
where
    Set: cote::prelude::Set + cote::prelude::OptParser + cote::prelude::OptValidator
        + cote::prelude::SetValueFindExt + Default + 'inv,
    Ser: cote::prelude::ServicesValExt + Default + 'inv,
    cote::prelude::SetCfg<Set>: cote::prelude::ConfigValue + Default,
    <Set as cote::prelude::OptParser>::Output: cote::prelude::Information,
{
    fn update(parser: &mut cote::prelude::Parser<'inv, Set, Ser>) -> cote::Result<()> {
        type InferedOptVal<T> = <T as cote::prelude::Infer>::Val;
        let set = parser.optset_mut();
        let ctor_name = cote::prelude::ctor_default_name();
        let option_0 = {
            let cfg = {
                let mut cfg = cote::prelude::SetCfg::<Set>::default();
                cote::prelude::ConfigValue::set_name(&mut cfg, "c");
                <cote::prelude::Cmd as cote::prelude::Infer>::infer_fill_info(&mut cfg)?;
                cfg
            };
            cote::prelude::Ctor::new_with(
                    cote::prelude::SetExt::ctor_mut(set, &ctor_name)?,
                    cfg,
                )
                .map_err(Into::into)?
        };
        let option_1 = {
            let cfg = {
                let mut cfg = cote::prelude::SetCfg::<Set>::default();
                cote::prelude::ConfigValue::set_name(&mut cfg, "cpp");
                cote::prelude::ConfigValue::add_alias(&mut cfg, "cxx");
                cote::prelude::ConfigValue::add_alias(&mut cfg, "cc");
                <cote::prelude::Cmd as cote::prelude::Infer>::infer_fill_info(&mut cfg)?;
                cfg
            };
            cote::prelude::Ctor::new_with(
                    cote::prelude::SetExt::ctor_mut(set, &ctor_name)?,
                    cfg,
                )
                .map_err(Into::into)?
        };
        let option_2 = {
            let cfg = {
                let mut cfg = cote::prelude::SetCfg::<Set>::default();
                cote::prelude::ConfigValue::set_name(&mut cfg, "rust");
                cote::prelude::ConfigValue::add_alias(&mut cfg, "rs");
                <cote::prelude::Cmd as cote::prelude::Infer>::infer_fill_info(&mut cfg)?;
                cfg
            };
            cote::prelude::Ctor::new_with(
                    cote::prelude::SetExt::ctor_mut(set, &ctor_name)?,
                    cfg,
                )
                .map_err(Into::into)?
        };
        let option_0_uid = set.insert(option_0);
        match (&option_0_uid, &0u64) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::Some(
                            format_args!("Oops! Uid must be equal here"),
                        ),
                    );
                }
            }
        };
        let option_1_uid = set.insert(option_1);
        match (&option_1_uid, &1u64) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::Some(
                            format_args!("Oops! Uid must be equal here"),
                        ),
                    );
                }
            }
        };
        let option_2_uid = set.insert(option_2);
        match (&option_2_uid, &2u64) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::Some(
                            format_args!("Oops! Uid must be equal here"),
                        ),
                    );
                }
            }
        };
        parser
            .entry(option_0_uid)?
            .on(move |
                set: &mut cote::prelude::Parser<'inv, Set, Ser>,
                ser: &mut Ser,
                args: cote::prelude::ctx::Args,
                index: cote::prelude::ctx::Index|
            {
                use std::ops::Deref;
                let mut args: Vec<cote::prelude::RawVal> = args.deref().clone().into();
                let cmd = args.remove(*index.deref());
                let cmd = cmd.get_str();
                let cmd = cmd
                    .ok_or_else(|| ::aopt::Error::raise_error({
                        let res = ::alloc::fmt::format(
                            format_args!("Can not convert `{0:?}` to &str", cmd),
                        );
                        res
                    }))?;
                let args = cote::prelude::ARef::new(cote::prelude::Args::from(args));
                let parser = set.parser_mut(0)?;
                let mut policy = <cote::prelude::FwdPolicy<'inv, Set, Ser>>::default();
                let name = parser.name().clone();
                parser.set_rctx(ser.sve_take_val::<cote::prelude::RunningCtx>()?);
                parser.rctx_mut()?.add_name(name);
                <C>::apply_policy_settings(&mut policy);
                let ret = cote::prelude::PolicyParser::parse_policy(
                    parser,
                    args,
                    &mut policy,
                );
                let mut rctx = parser.take_rctx()?;
                if !rctx.display_help() {
                    <C>::sync_rctx(&mut rctx, &ret, parser.optset(), true)?;
                    if rctx.display_help() {
                        rctx.set_help_context(<C>::new_help_context());
                    } else {
                        rctx.pop_name();
                    }
                }
                rctx.set_sub_parser(true);
                ser.sve_insert(rctx);
                let ret = ret?;
                let okay = ret.status();
                Ok(
                    if okay {
                        ser.sve_val_mut::<cote::prelude::RunningCtx>()?
                            .clear_failed_info();
                        <C as cote::ExtractFromSetDerive<
                            Set,
                        >>::try_extract(parser.optset_mut())
                            .ok()
                    } else {
                        ser.sve_val_mut::<cote::prelude::RunningCtx>()?
                            .add_failed_info(cote::prelude::FailedInfo {
                                name: cmd.to_owned(),
                                retval: ret,
                            });
                        None
                    },
                )
            });
        parser
            .entry(option_1_uid)?
            .on(move |
                set: &mut cote::prelude::Parser<'inv, Set, Ser>,
                ser: &mut Ser,
                args: cote::prelude::ctx::Args,
                index: cote::prelude::ctx::Index|
            {
                use std::ops::Deref;
                let mut args: Vec<cote::prelude::RawVal> = args.deref().clone().into();
                let cmd = args.remove(*index.deref());
                let cmd = cmd.get_str();
                let cmd = cmd
                    .ok_or_else(|| ::aopt::Error::raise_error({
                        let res = ::alloc::fmt::format(
                            format_args!("Can not convert `{0:?}` to &str", cmd),
                        );
                        res
                    }))?;
                let args = cote::prelude::ARef::new(cote::prelude::Args::from(args));
                let parser = set.parser_mut(1)?;
                let mut policy = <cote::prelude::FwdPolicy<'inv, Set, Ser>>::default();
                let name = parser.name().clone();
                parser.set_rctx(ser.sve_take_val::<cote::prelude::RunningCtx>()?);
                parser.rctx_mut()?.add_name(name);
                <Cpp>::apply_policy_settings(&mut policy);
                let ret = cote::prelude::PolicyParser::parse_policy(
                    parser,
                    args,
                    &mut policy,
                );
                let mut rctx = parser.take_rctx()?;
                if !rctx.display_help() {
                    <Cpp>::sync_rctx(&mut rctx, &ret, parser.optset(), true)?;
                    if rctx.display_help() {
                        rctx.set_help_context(<Cpp>::new_help_context());
                    } else {
                        rctx.pop_name();
                    }
                }
                rctx.set_sub_parser(true);
                ser.sve_insert(rctx);
                let ret = ret?;
                let okay = ret.status();
                Ok(
                    if okay {
                        ser.sve_val_mut::<cote::prelude::RunningCtx>()?
                            .clear_failed_info();
                        <Cpp as cote::ExtractFromSetDerive<
                            Set,
                        >>::try_extract(parser.optset_mut())
                            .ok()
                    } else {
                        ser.sve_val_mut::<cote::prelude::RunningCtx>()?
                            .add_failed_info(cote::prelude::FailedInfo {
                                name: cmd.to_owned(),
                                retval: ret,
                            });
                        None
                    },
                )
            });
        parser
            .entry(option_2_uid)?
            .on(move |
                set: &mut cote::prelude::Parser<'inv, Set, Ser>,
                ser: &mut Ser,
                args: cote::prelude::ctx::Args,
                index: cote::prelude::ctx::Index|
            {
                use std::ops::Deref;
                let mut args: Vec<cote::prelude::RawVal> = args.deref().clone().into();
                let cmd = args.remove(*index.deref());
                let cmd = cmd.get_str();
                let cmd = cmd
                    .ok_or_else(|| ::aopt::Error::raise_error({
                        let res = ::alloc::fmt::format(
                            format_args!("Can not convert `{0:?}` to &str", cmd),
                        );
                        res
                    }))?;
                let args = cote::prelude::ARef::new(cote::prelude::Args::from(args));
                let parser = set.parser_mut(2)?;
                let mut policy = <cote::prelude::FwdPolicy<'inv, Set, Ser>>::default();
                let name = parser.name().clone();
                parser.set_rctx(ser.sve_take_val::<cote::prelude::RunningCtx>()?);
                parser.rctx_mut()?.add_name(name);
                <Rust>::apply_policy_settings(&mut policy);
                let ret = cote::prelude::PolicyParser::parse_policy(
                    parser,
                    args,
                    &mut policy,
                );
                let mut rctx = parser.take_rctx()?;
                if !rctx.display_help() {
                    <Rust>::sync_rctx(&mut rctx, &ret, parser.optset(), true)?;
                    if rctx.display_help() {
                        rctx.set_help_context(<Rust>::new_help_context());
                    } else {
                        rctx.pop_name();
                    }
                }
                rctx.set_sub_parser(true);
                ser.sve_insert(rctx);
                let ret = ret?;
                let okay = ret.status();
                Ok(
                    if okay {
                        ser.sve_val_mut::<cote::prelude::RunningCtx>()?
                            .clear_failed_info();
                        <Rust as cote::ExtractFromSetDerive<
                            Set,
                        >>::try_extract(parser.optset_mut())
                            .ok()
                    } else {
                        ser.sve_val_mut::<cote::prelude::RunningCtx>()?
                            .add_failed_info(cote::prelude::FailedInfo {
                                name: cmd.to_owned(),
                                retval: ret,
                            });
                        None
                    },
                )
            });
        Ok(())
    }
}
///Automatic generated by cote-derive for [`Snippet`].
impl<'set, Set> cote::ExtractFromSetDerive<'set, Set> for Snippet
where
    Set: cote::prelude::SetValueFindExt,
    cote::prelude::SetCfg<Set>: cote::prelude::ConfigValue + Default,
{
    fn try_extract(set: &'set mut Set) -> cote::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            c: cote::prelude::fetch_uid_impl(0u64, set).ok(),
            cpp: cote::prelude::fetch_uid_impl(1u64, set).ok(),
            rust: cote::prelude::fetch_uid_impl(2u64, set).ok(),
        })
    }
}
///Automatic generated by cote-derive for [`Snippet`].
impl Snippet {
    #[doc(hidden)]
    pub fn new_help_context() -> cote::prelude::HelpContext {
        cote::prelude::HelpContext::default()
            .with_name(String::from("snippet"))
            .with_head(String::from(""))
            .with_foot({
                let res = ::alloc::fmt::format(
                    format_args!("Create by {0} v{1}", "araraloren", "0.1.0"),
                );
                res
            })
            .with_width(40usize)
            .with_usagew(10usize)
    }
    #[doc(hidden)]
    pub fn sync_rctx<'a, Set, Ret>(
        rctx: &'a mut cote::prelude::RunningCtx,
        ret: &cote::Result<Ret>,
        set: &Set,
        sub_parser: bool,
    ) -> cote::Result<&'a mut cote::prelude::RunningCtx>
    where
        Set: cote::prelude::SetValueFindExt,
        Ret: cote::prelude::Status,
        cote::prelude::SetCfg<Set>: cote::prelude::ConfigValue + Default,
    {
        if !ret.is_ok()
            || !ret.as_ref().map(cote::prelude::Status::status).unwrap_or(true)
        {
            rctx.set_display_help(true);
            rctx.set_exit(false);
        }
        Ok(rctx)
    }
    pub fn into_parser<'inv>() -> cote::Result<
        cote::prelude::Parser<'inv, cote::prelude::ASet, cote::prelude::ASer>,
    > {
        Self::into_parser_with::<cote::prelude::ASet, cote::prelude::ASer>()
    }
    pub fn into_parser_with<'inv, Set, Ser>() -> cote::Result<
        cote::prelude::Parser<'inv, Set, Ser>,
    >
    where
        Ser: cote::prelude::ServicesValExt + Default + 'inv,
        cote::prelude::SetCfg<Set>: cote::prelude::ConfigValue + Default,
        <Set as cote::prelude::OptParser>::Output: cote::prelude::Information,
        Set: cote::prelude::Set + cote::prelude::OptParser + cote::prelude::OptValidator
            + cote::prelude::SetValueFindExt + Default + 'inv,
    {
        let mut parser = <Self as cote::IntoParserDerive<
            'inv,
            Set,
            Ser,
        >>::into_parser()?;
        parser.add_parser(<C>::into_parser_with::<Set, Ser>()?.with_name("c"));
        parser.add_parser(<Cpp>::into_parser_with::<Set, Ser>()?.with_name("cpp"));
        parser.add_parser(<Rust>::into_parser_with::<Set, Ser>()?.with_name("rust"));
        Ok(parser.with_name(String::from("snippet")))
    }
    pub fn into_policy<'inv>() -> cote::prelude::PrePolicy<
        'inv,
        cote::prelude::ASet,
        cote::prelude::ASer,
    > {
        Self::into_policy_with()
    }
    pub fn into_policy_with<'inv, Set, Ser>() -> cote::prelude::PrePolicy<
        'inv,
        Set,
        Ser,
    > {
        let mut policy: cote::prelude::PrePolicy<'inv, Set, Ser> = Default::default();
        Self::apply_policy_settings(&mut policy);
        policy
    }
    pub fn apply_policy_settings(policy: &mut impl cote::prelude::PolicySettings) {
        let style_manager = policy.style_manager_mut();
    }
    pub fn parse_args_with<'inv, Set, Ser, P>(
        args: cote::prelude::Args,
        policy: &mut P,
    ) -> cote::Result<cote::prelude::CoteRes<&mut P, P>>
    where
        P::Error: Into<cote::Error>,
        P::Ret: cote::prelude::Status,
        Ser: cote::prelude::ServicesValExt + Default + 'inv,
        cote::prelude::SetCfg<Set>: cote::prelude::ConfigValue + Default,
        <Set as cote::prelude::OptParser>::Output: cote::prelude::Information,
        Set: cote::prelude::Set + cote::prelude::OptParser + cote::prelude::OptValidator
            + cote::prelude::SetValueFindExt + Default + 'inv,
        P: cote::prelude::Policy<
                Set = cote::prelude::Parser<'inv, Set, Ser>,
                Ser = Ser,
                Inv<
                    'inv,
                > = cote::prelude::Invoker<
                    'inv,
                    cote::prelude::Parser<'inv, Set, Ser>,
                    Ser,
                >,
            > + cote::prelude::APolicyExt<P> + cote::prelude::PolicySettings + Default,
    {
        let mut parser = Self::into_parser_with::<'inv, Set, Ser>()?;
        let mut rctx = cote::prelude::RunningCtx::default();
        rctx.add_name(String::from("snippet"));
        parser.set_rctx(rctx);
        let args = cote::prelude::ARef::new(args);
        let ret = cote::prelude::PolicyParser::parse_policy(&mut parser, args, policy);
        let mut rctx = parser.take_rctx()?;
        if !rctx.display_help() {
            Self::sync_rctx::<Set, _>(&mut rctx, &ret, parser.optset(), false)?;
            if rctx.display_help() {
                rctx.set_help_context(Self::new_help_context());
            }
        }
        parser.set_rctx(rctx);
        let mut rctx = parser.rctx()?;
        if rctx.display_help() {
            let names = rctx.names().iter().map(|v| v.as_str()).collect::<Vec<&str>>();
            let help_context = rctx.help_context().unwrap();
            let exit = rctx.exit();
            parser.display_sub_help(names, &help_context)?;
            if exit {
                std::process::exit(0);
            }
        }
        Ok(cote::prelude::CoteRes {
            ret: ret?,
            parser,
            policy,
        })
    }
    pub fn parse_args<'inv>(
        args: cote::prelude::Args,
    ) -> cote::Result<
        cote::prelude::CoteRes<
            cote::prelude::PrePolicy<'inv, cote::prelude::ASet, cote::prelude::ASer>,
            cote::prelude::PrePolicy<'inv, cote::prelude::ASet, cote::prelude::ASer>,
        >,
    > {
        let mut policy = Self::into_policy();
        let cote::prelude::CoteRes { ret, parser, .. } = Self::parse_args_with(
            args,
            &mut policy,
        )?;
        Ok(cote::prelude::CoteRes {
            ret,
            parser,
            policy,
        })
    }
    pub fn parse(args: cote::prelude::Args) -> cote::Result<Self> {
        let cote::prelude::CoteRes { mut ret, mut parser, .. } = Self::parse_args(args)?;
        if let Some(mut error) = ret.take_failure() {
            let mut rctx = parser.take_rctx()?;
            if let Some(chain_error) = rctx.chain_error() {
                error = error.cause_by(chain_error);
            }
            let mut failed_info = rctx.take_failed_info();
            let (command, ret) = failed_info
                .last_mut()
                .map(|v| (Some(v.name.as_str()), &mut v.retval))
                .unwrap_or((None, &mut ret));
            let e = {
                let ctx = ret.take_ctx();
                let args = ctx
                    .orig_args()[1..]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                let inner_ctx = ctx.inner_ctx().ok();
                let failed_msg = if let Some(command) = command {
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("Parsing command `{0}`", command),
                        );
                        res
                    }
                } else {
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("Parsing arguments `{0}`", args),
                        );
                        res
                    }
                };
                let inner_ctx = if let Some(inner_ctx) = inner_ctx {
                    {
                        let res = ::alloc::fmt::format(format_args!("{0}", inner_ctx));
                        res
                    }
                } else {
                    "None".to_owned()
                };
                ::aopt::Error::raise_failure({
                        let res = ::alloc::fmt::format(
                            format_args!("{0} failed: {1}", failed_msg, inner_ctx),
                        );
                        res
                    })
                    .cause_by(error)
            };
            Err(e)
        } else {
            <Self as cote::ExtractFromSetDerive<
                cote::prelude::ASet,
            >>::try_extract(parser.optset_mut())
        }
    }
    pub fn parse_env_args_with<'inv, Set, Ser, P>(
        policy: &mut P,
    ) -> cote::Result<cote::prelude::CoteRes<&mut P, P>>
    where
        P::Error: Into<cote::Error>,
        P::Ret: cote::prelude::Status,
        Ser: cote::prelude::ServicesValExt + Default + 'inv,
        cote::prelude::SetCfg<Set>: cote::prelude::ConfigValue + Default,
        <Set as cote::prelude::OptParser>::Output: cote::prelude::Information,
        Set: cote::prelude::Set + cote::prelude::OptParser + cote::prelude::OptValidator
            + cote::prelude::SetValueFindExt + Default + 'inv,
        P: cote::prelude::Policy<
                Set = cote::prelude::Parser<'inv, Set, Ser>,
                Ser = Ser,
                Inv<
                    'inv,
                > = cote::prelude::Invoker<
                    'inv,
                    cote::prelude::Parser<'inv, Set, Ser>,
                    Ser,
                >,
            > + cote::prelude::APolicyExt<P> + cote::prelude::PolicySettings + Default,
    {
        Self::parse_args_with(cote::prelude::Args::from_env(), policy)
    }
    pub fn parse_env_args<'inv>() -> cote::Result<
        cote::prelude::CoteRes<
            cote::prelude::PrePolicy<'inv, cote::prelude::ASet, cote::prelude::ASer>,
            cote::prelude::PrePolicy<'inv, cote::prelude::ASet, cote::prelude::ASer>,
        >,
    > {
        Self::parse_args(cote::prelude::Args::from_env())
    }
    pub fn parse_env() -> cote::Result<Self> {
        Self::parse(cote::prelude::Args::from_env())
    }
}
fn main() -> color_eyre::Result<()> {
    let body = async {
        let mut parser = Snippet::into_parser()?;
        Ok(
            parser
                .run_async_mut(
                    &mut PrePolicy::default(),
                    |r, p| async {
                        if r.status() {
                            run_compiler(r, p)
                                .await
                                .map_err(|e| ::aopt::Error::raise_error({
                                    let res = ::alloc::fmt::format(
                                        format_args!("running command failed: {0:?}", e),
                                    );
                                    res
                                }))
                        } else {
                            ::core::panicking::panic("not yet implemented")
                        }
                    },
                )
                .await?,
        )
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
pub async fn run_compiler(
    ret: ReturnVal,
    parser: &mut Parser<'static, ASet, ASer>,
) -> wasmtime::Result<()> {
    let data: &[u8] = ::core::panicking::panic("not yet implemented");
    let mut config = Config::new();
    let engine = wasmtime::Engine::new(config.async_support(true))?;
    let mut linker = Linker::<State>::new(&engine);
    let closure = type_annotate::<State, _>(|t| WasiImpl(t));
    wasmtime_wasi::add_to_linker_async(&mut linker)?;
    host::types::add_to_linker_get_host(&mut linker, closure)?;
    let mut store = Store::new(
        &engine,
        State {
            ctx: WasiCtxBuilder::new().inherit_stdin().inherit_stdout().build(),
            table: ResourceTable::new(),
        },
    );
    let lang = Component::from_binary(&engine, &data)?;
    let bindings = Root::instantiate_async(&mut store, &lang, &linker).await?;
    let result = bindings.snippet_plugin_language().call_name(&mut store).await?;
    {
        ::std::io::_print(format_args!("Greeting: {0:?}\n", result));
    };
    let langs = bindings.snippet_plugin_compiler().call_support(&mut store).await?;
    {
        ::std::io::_print(format_args!("--> supports: {0:?}\n", langs));
    };
    let optset = bindings
        .snippet_plugin_language()
        .call_initialize_optset(&mut store)
        .await??;
    bindings.snippet_plugin_language().call_fill_optset(&mut store, optset).await??;
    let optset = bindings
        .snippet_plugin_language()
        .call_initialize_optset(&mut store)
        .await??;
    let complier = bindings
        .snippet_plugin_compiler()
        .compiler()
        .call_constructor(&mut store)
        .await?;
    let res = bindings
        .snippet_plugin_language()
        .call_compile(&mut store, optset, complier)
        .await??;
    match res {
        tmp => {
            {
                ::std::io::_eprint(
                    format_args!(
                        "[{0}:{1}:{2}] {3} = {4:#?}\n",
                        "packages\\snippet\\src/main.rs",
                        155u32,
                        5u32,
                        "res",
                        &tmp,
                    ),
                );
            };
            tmp
        }
    };
    Ok(())
}

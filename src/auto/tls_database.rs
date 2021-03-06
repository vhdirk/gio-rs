// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "futures", feature = "dox"))]
use futures::future;
use gio_sys;
use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::ptr;
use Cancellable;
use Error;
use SocketConnectable;
use TlsCertificate;
use TlsCertificateFlags;
use TlsDatabaseLookupFlags;
use TlsDatabaseVerifyFlags;
use TlsInteraction;

glib_wrapper! {
    pub struct TlsDatabase(Object<gio_sys::GTlsDatabase, gio_sys::GTlsDatabaseClass, TlsDatabaseClass>);

    match fn {
        get_type => || gio_sys::g_tls_database_get_type(),
    }
}

pub const NONE_TLS_DATABASE: Option<&TlsDatabase> = None;

pub trait TlsDatabaseExt: 'static {
    fn create_certificate_handle<P: IsA<TlsCertificate>>(&self, certificate: &P)
        -> Option<GString>;

    fn lookup_certificate_for_handle<P: IsA<TlsInteraction>, Q: IsA<Cancellable>>(
        &self,
        handle: &str,
        interaction: Option<&P>,
        flags: TlsDatabaseLookupFlags,
        cancellable: Option<&Q>,
    ) -> Result<Option<TlsCertificate>, Error>;

    fn lookup_certificate_for_handle_async<
        P: IsA<TlsInteraction>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<TlsCertificate, Error>) + Send + 'static,
    >(
        &self,
        handle: &str,
        interaction: Option<&P>,
        flags: TlsDatabaseLookupFlags,
        cancellable: Option<&Q>,
        callback: R,
    );

    #[cfg(any(feature = "futures", feature = "dox"))]
    fn lookup_certificate_for_handle_async_future<P: IsA<TlsInteraction> + Clone + 'static>(
        &self,
        handle: &str,
        interaction: Option<&P>,
        flags: TlsDatabaseLookupFlags,
    ) -> Box_<dyn future::Future<Output = Result<TlsCertificate, Error>> + std::marker::Unpin>;

    fn lookup_certificate_issuer<
        P: IsA<TlsCertificate>,
        Q: IsA<TlsInteraction>,
        R: IsA<Cancellable>,
    >(
        &self,
        certificate: &P,
        interaction: Option<&Q>,
        flags: TlsDatabaseLookupFlags,
        cancellable: Option<&R>,
    ) -> Result<TlsCertificate, Error>;

    fn lookup_certificate_issuer_async<
        P: IsA<TlsCertificate>,
        Q: IsA<TlsInteraction>,
        R: IsA<Cancellable>,
        S: FnOnce(Result<TlsCertificate, Error>) + Send + 'static,
    >(
        &self,
        certificate: &P,
        interaction: Option<&Q>,
        flags: TlsDatabaseLookupFlags,
        cancellable: Option<&R>,
        callback: S,
    );

    #[cfg(any(feature = "futures", feature = "dox"))]
    fn lookup_certificate_issuer_async_future<
        P: IsA<TlsCertificate> + Clone + 'static,
        Q: IsA<TlsInteraction> + Clone + 'static,
    >(
        &self,
        certificate: &P,
        interaction: Option<&Q>,
        flags: TlsDatabaseLookupFlags,
    ) -> Box_<dyn future::Future<Output = Result<TlsCertificate, Error>> + std::marker::Unpin>;

    //fn lookup_certificates_issued_by<P: IsA<TlsInteraction>, Q: IsA<Cancellable>>(&self, issuer_raw_dn: /*Ignored*/&glib::ByteArray, interaction: Option<&P>, flags: TlsDatabaseLookupFlags, cancellable: Option<&Q>) -> Result<Vec<TlsCertificate>, Error>;

    //fn lookup_certificates_issued_by_async<P: IsA<TlsInteraction>, Q: IsA<Cancellable>, R: FnOnce(Result<Vec<TlsCertificate>, Error>) + Send + 'static>(&self, issuer_raw_dn: /*Ignored*/&glib::ByteArray, interaction: Option<&P>, flags: TlsDatabaseLookupFlags, cancellable: Option<&Q>, callback: R);

    //#[cfg(any(feature = "futures", feature = "dox"))]
    //fn lookup_certificates_issued_by_async_future<P: IsA<TlsInteraction> + Clone + 'static>(&self, issuer_raw_dn: /*Ignored*/&glib::ByteArray, interaction: Option<&P>, flags: TlsDatabaseLookupFlags) -> Box_<dyn future::Future<Output = Result<Vec<TlsCertificate>, Error>> + std::marker::Unpin>;

    fn verify_chain<
        P: IsA<TlsCertificate>,
        Q: IsA<SocketConnectable>,
        R: IsA<TlsInteraction>,
        S: IsA<Cancellable>,
    >(
        &self,
        chain: &P,
        purpose: &str,
        identity: Option<&Q>,
        interaction: Option<&R>,
        flags: TlsDatabaseVerifyFlags,
        cancellable: Option<&S>,
    ) -> Result<TlsCertificateFlags, Error>;

    fn verify_chain_async<
        P: IsA<TlsCertificate>,
        Q: IsA<SocketConnectable>,
        R: IsA<TlsInteraction>,
        S: IsA<Cancellable>,
        T: FnOnce(Result<TlsCertificateFlags, Error>) + Send + 'static,
    >(
        &self,
        chain: &P,
        purpose: &str,
        identity: Option<&Q>,
        interaction: Option<&R>,
        flags: TlsDatabaseVerifyFlags,
        cancellable: Option<&S>,
        callback: T,
    );

    #[cfg(any(feature = "futures", feature = "dox"))]
    fn verify_chain_async_future<
        P: IsA<TlsCertificate> + Clone + 'static,
        Q: IsA<SocketConnectable> + Clone + 'static,
        R: IsA<TlsInteraction> + Clone + 'static,
    >(
        &self,
        chain: &P,
        purpose: &str,
        identity: Option<&Q>,
        interaction: Option<&R>,
        flags: TlsDatabaseVerifyFlags,
    ) -> Box_<dyn future::Future<Output = Result<TlsCertificateFlags, Error>> + std::marker::Unpin>;
}

impl<O: IsA<TlsDatabase>> TlsDatabaseExt for O {
    fn create_certificate_handle<P: IsA<TlsCertificate>>(
        &self,
        certificate: &P,
    ) -> Option<GString> {
        unsafe {
            from_glib_full(gio_sys::g_tls_database_create_certificate_handle(
                self.as_ref().to_glib_none().0,
                certificate.as_ref().to_glib_none().0,
            ))
        }
    }

    fn lookup_certificate_for_handle<P: IsA<TlsInteraction>, Q: IsA<Cancellable>>(
        &self,
        handle: &str,
        interaction: Option<&P>,
        flags: TlsDatabaseLookupFlags,
        cancellable: Option<&Q>,
    ) -> Result<Option<TlsCertificate>, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_tls_database_lookup_certificate_for_handle(
                self.as_ref().to_glib_none().0,
                handle.to_glib_none().0,
                interaction.map(|p| p.as_ref()).to_glib_none().0,
                flags.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn lookup_certificate_for_handle_async<
        P: IsA<TlsInteraction>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<TlsCertificate, Error>) + Send + 'static,
    >(
        &self,
        handle: &str,
        interaction: Option<&P>,
        flags: TlsDatabaseLookupFlags,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let user_data: Box_<R> = Box_::new(callback);
        unsafe extern "C" fn lookup_certificate_for_handle_async_trampoline<
            R: FnOnce(Result<TlsCertificate, Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_tls_database_lookup_certificate_for_handle_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<R> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = lookup_certificate_for_handle_async_trampoline::<R>;
        unsafe {
            gio_sys::g_tls_database_lookup_certificate_for_handle_async(
                self.as_ref().to_glib_none().0,
                handle.to_glib_none().0,
                interaction.map(|p| p.as_ref()).to_glib_none().0,
                flags.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(any(feature = "futures", feature = "dox"))]
    fn lookup_certificate_for_handle_async_future<P: IsA<TlsInteraction> + Clone + 'static>(
        &self,
        handle: &str,
        interaction: Option<&P>,
        flags: TlsDatabaseLookupFlags,
    ) -> Box_<dyn future::Future<Output = Result<TlsCertificate, Error>> + std::marker::Unpin> {
        use fragile::Fragile;
        use GioFuture;

        let handle = String::from(handle);
        let interaction = interaction.map(ToOwned::to_owned);
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            obj.lookup_certificate_for_handle_async(
                &handle,
                interaction.as_ref().map(::std::borrow::Borrow::borrow),
                flags,
                Some(&cancellable),
                move |res| {
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }

    fn lookup_certificate_issuer<
        P: IsA<TlsCertificate>,
        Q: IsA<TlsInteraction>,
        R: IsA<Cancellable>,
    >(
        &self,
        certificate: &P,
        interaction: Option<&Q>,
        flags: TlsDatabaseLookupFlags,
        cancellable: Option<&R>,
    ) -> Result<TlsCertificate, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_tls_database_lookup_certificate_issuer(
                self.as_ref().to_glib_none().0,
                certificate.as_ref().to_glib_none().0,
                interaction.map(|p| p.as_ref()).to_glib_none().0,
                flags.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn lookup_certificate_issuer_async<
        P: IsA<TlsCertificate>,
        Q: IsA<TlsInteraction>,
        R: IsA<Cancellable>,
        S: FnOnce(Result<TlsCertificate, Error>) + Send + 'static,
    >(
        &self,
        certificate: &P,
        interaction: Option<&Q>,
        flags: TlsDatabaseLookupFlags,
        cancellable: Option<&R>,
        callback: S,
    ) {
        let user_data: Box_<S> = Box_::new(callback);
        unsafe extern "C" fn lookup_certificate_issuer_async_trampoline<
            S: FnOnce(Result<TlsCertificate, Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_tls_database_lookup_certificate_issuer_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<S> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = lookup_certificate_issuer_async_trampoline::<S>;
        unsafe {
            gio_sys::g_tls_database_lookup_certificate_issuer_async(
                self.as_ref().to_glib_none().0,
                certificate.as_ref().to_glib_none().0,
                interaction.map(|p| p.as_ref()).to_glib_none().0,
                flags.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(any(feature = "futures", feature = "dox"))]
    fn lookup_certificate_issuer_async_future<
        P: IsA<TlsCertificate> + Clone + 'static,
        Q: IsA<TlsInteraction> + Clone + 'static,
    >(
        &self,
        certificate: &P,
        interaction: Option<&Q>,
        flags: TlsDatabaseLookupFlags,
    ) -> Box_<dyn future::Future<Output = Result<TlsCertificate, Error>> + std::marker::Unpin> {
        use fragile::Fragile;
        use GioFuture;

        let certificate = certificate.clone();
        let interaction = interaction.map(ToOwned::to_owned);
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            obj.lookup_certificate_issuer_async(
                &certificate,
                interaction.as_ref().map(::std::borrow::Borrow::borrow),
                flags,
                Some(&cancellable),
                move |res| {
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }

    //fn lookup_certificates_issued_by<P: IsA<TlsInteraction>, Q: IsA<Cancellable>>(&self, issuer_raw_dn: /*Ignored*/&glib::ByteArray, interaction: Option<&P>, flags: TlsDatabaseLookupFlags, cancellable: Option<&Q>) -> Result<Vec<TlsCertificate>, Error> {
    //    unsafe { TODO: call gio_sys:g_tls_database_lookup_certificates_issued_by() }
    //}

    //fn lookup_certificates_issued_by_async<P: IsA<TlsInteraction>, Q: IsA<Cancellable>, R: FnOnce(Result<Vec<TlsCertificate>, Error>) + Send + 'static>(&self, issuer_raw_dn: /*Ignored*/&glib::ByteArray, interaction: Option<&P>, flags: TlsDatabaseLookupFlags, cancellable: Option<&Q>, callback: R) {
    //    unsafe { TODO: call gio_sys:g_tls_database_lookup_certificates_issued_by_async() }
    //}

    //#[cfg(any(feature = "futures", feature = "dox"))]
    //fn lookup_certificates_issued_by_async_future<P: IsA<TlsInteraction> + Clone + 'static>(&self, issuer_raw_dn: /*Ignored*/&glib::ByteArray, interaction: Option<&P>, flags: TlsDatabaseLookupFlags) -> Box_<dyn future::Future<Output = Result<Vec<TlsCertificate>, Error>> + std::marker::Unpin> {
    //use GioFuture;
    //use fragile::Fragile;

    //let issuer_raw_dn = issuer_raw_dn.clone();
    //let interaction = interaction.map(ToOwned::to_owned);
    //GioFuture::new(self, move |obj, send| {
    //    let cancellable = Cancellable::new();
    //    let send = Fragile::new(send);
    //    obj.lookup_certificates_issued_by_async(
    //        &issuer_raw_dn,
    //        interaction.as_ref().map(::std::borrow::Borrow::borrow),
    //        flags,
    //        Some(&cancellable),
    //        move |res| {
    //            let _ = send.into_inner().send(res);
    //        },
    //    );

    //    cancellable
    //})
    //}

    fn verify_chain<
        P: IsA<TlsCertificate>,
        Q: IsA<SocketConnectable>,
        R: IsA<TlsInteraction>,
        S: IsA<Cancellable>,
    >(
        &self,
        chain: &P,
        purpose: &str,
        identity: Option<&Q>,
        interaction: Option<&R>,
        flags: TlsDatabaseVerifyFlags,
        cancellable: Option<&S>,
    ) -> Result<TlsCertificateFlags, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_tls_database_verify_chain(
                self.as_ref().to_glib_none().0,
                chain.as_ref().to_glib_none().0,
                purpose.to_glib_none().0,
                identity.map(|p| p.as_ref()).to_glib_none().0,
                interaction.map(|p| p.as_ref()).to_glib_none().0,
                flags.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn verify_chain_async<
        P: IsA<TlsCertificate>,
        Q: IsA<SocketConnectable>,
        R: IsA<TlsInteraction>,
        S: IsA<Cancellable>,
        T: FnOnce(Result<TlsCertificateFlags, Error>) + Send + 'static,
    >(
        &self,
        chain: &P,
        purpose: &str,
        identity: Option<&Q>,
        interaction: Option<&R>,
        flags: TlsDatabaseVerifyFlags,
        cancellable: Option<&S>,
        callback: T,
    ) {
        let user_data: Box_<T> = Box_::new(callback);
        unsafe extern "C" fn verify_chain_async_trampoline<
            T: FnOnce(Result<TlsCertificateFlags, Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_tls_database_verify_chain_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<T> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = verify_chain_async_trampoline::<T>;
        unsafe {
            gio_sys::g_tls_database_verify_chain_async(
                self.as_ref().to_glib_none().0,
                chain.as_ref().to_glib_none().0,
                purpose.to_glib_none().0,
                identity.map(|p| p.as_ref()).to_glib_none().0,
                interaction.map(|p| p.as_ref()).to_glib_none().0,
                flags.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(any(feature = "futures", feature = "dox"))]
    fn verify_chain_async_future<
        P: IsA<TlsCertificate> + Clone + 'static,
        Q: IsA<SocketConnectable> + Clone + 'static,
        R: IsA<TlsInteraction> + Clone + 'static,
    >(
        &self,
        chain: &P,
        purpose: &str,
        identity: Option<&Q>,
        interaction: Option<&R>,
        flags: TlsDatabaseVerifyFlags,
    ) -> Box_<dyn future::Future<Output = Result<TlsCertificateFlags, Error>> + std::marker::Unpin>
    {
        use fragile::Fragile;
        use GioFuture;

        let chain = chain.clone();
        let purpose = String::from(purpose);
        let identity = identity.map(ToOwned::to_owned);
        let interaction = interaction.map(ToOwned::to_owned);
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            obj.verify_chain_async(
                &chain,
                &purpose,
                identity.as_ref().map(::std::borrow::Borrow::borrow),
                interaction.as_ref().map(::std::borrow::Borrow::borrow),
                flags,
                Some(&cancellable),
                move |res| {
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }
}

impl fmt::Display for TlsDatabase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TlsDatabase")
    }
}

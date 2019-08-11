// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(feature = "futures")]
use futures::future;
use gio_sys;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;
use Cancellable;
use Error;
use IOStream;
use ProxyResolver;
use SocketAddress;
use SocketClientEvent;
use SocketConnectable;
use SocketConnection;
use SocketFamily;
use SocketProtocol;
use SocketType;
use TlsCertificateFlags;

glib_wrapper! {
    pub struct SocketClient(Object<gio_sys::GSocketClient, gio_sys::GSocketClientClass, SocketClientClass>);

    match fn {
        get_type => || gio_sys::g_socket_client_get_type(),
    }
}

impl SocketClient {
    pub fn new() -> SocketClient {
        unsafe { from_glib_full(gio_sys::g_socket_client_new()) }
    }
}

impl Default for SocketClient {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SOCKET_CLIENT: Option<&SocketClient> = None;

pub trait SocketClientExt: 'static {
    fn add_application_proxy(&self, protocol: &str);

    fn connect<P: IsA<SocketConnectable>, Q: IsA<Cancellable>>(
        &self,
        connectable: &P,
        cancellable: Option<&Q>,
    ) -> Result<SocketConnection, Error>;

    fn connect_async<
        P: IsA<SocketConnectable>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<SocketConnection, Error>) + Send + 'static,
    >(
        &self,
        connectable: &P,
        cancellable: Option<&Q>,
        callback: R,
    );

    #[cfg(feature = "futures")]
    fn connect_async_future<P: IsA<SocketConnectable> + Clone + 'static>(
        &self,
        connectable: &P,
    ) -> Box_<dyn future::Future<Output = Result<SocketConnection, Error>> + std::marker::Unpin>;

    fn connect_to_host<P: IsA<Cancellable>>(
        &self,
        host_and_port: &str,
        default_port: u16,
        cancellable: Option<&P>,
    ) -> Result<SocketConnection, Error>;

    fn connect_to_host_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static,
    >(
        &self,
        host_and_port: &str,
        default_port: u16,
        cancellable: Option<&P>,
        callback: Q,
    );

    #[cfg(feature = "futures")]
    fn connect_to_host_async_future(
        &self,
        host_and_port: &str,
        default_port: u16,
    ) -> Box_<dyn future::Future<Output = Result<SocketConnection, Error>> + std::marker::Unpin>;

    fn connect_to_service<P: IsA<Cancellable>>(
        &self,
        domain: &str,
        service: &str,
        cancellable: Option<&P>,
    ) -> Result<SocketConnection, Error>;

    fn connect_to_service_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static,
    >(
        &self,
        domain: &str,
        service: &str,
        cancellable: Option<&P>,
        callback: Q,
    );

    #[cfg(feature = "futures")]
    fn connect_to_service_async_future(
        &self,
        domain: &str,
        service: &str,
    ) -> Box_<dyn future::Future<Output = Result<SocketConnection, Error>> + std::marker::Unpin>;

    fn connect_to_uri<P: IsA<Cancellable>>(
        &self,
        uri: &str,
        default_port: u16,
        cancellable: Option<&P>,
    ) -> Result<SocketConnection, Error>;

    fn connect_to_uri_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static,
    >(
        &self,
        uri: &str,
        default_port: u16,
        cancellable: Option<&P>,
        callback: Q,
    );

    #[cfg(feature = "futures")]
    fn connect_to_uri_async_future(
        &self,
        uri: &str,
        default_port: u16,
    ) -> Box_<dyn future::Future<Output = Result<SocketConnection, Error>> + std::marker::Unpin>;

    fn get_enable_proxy(&self) -> bool;

    fn get_family(&self) -> SocketFamily;

    fn get_local_address(&self) -> Option<SocketAddress>;

    fn get_protocol(&self) -> SocketProtocol;

    fn get_proxy_resolver(&self) -> Option<ProxyResolver>;

    fn get_socket_type(&self) -> SocketType;

    fn get_timeout(&self) -> u32;

    fn get_tls(&self) -> bool;

    fn get_tls_validation_flags(&self) -> TlsCertificateFlags;

    fn set_enable_proxy(&self, enable: bool);

    fn set_family(&self, family: SocketFamily);

    fn set_local_address<P: IsA<SocketAddress>>(&self, address: Option<&P>);

    fn set_protocol(&self, protocol: SocketProtocol);

    fn set_proxy_resolver<P: IsA<ProxyResolver>>(&self, proxy_resolver: Option<&P>);

    fn set_socket_type(&self, type_: SocketType);

    fn set_timeout(&self, timeout: u32);

    fn set_tls(&self, tls: bool);

    fn set_tls_validation_flags(&self, flags: TlsCertificateFlags);

    fn get_property_type(&self) -> SocketType;

    fn set_property_type(&self, type_: SocketType);

    fn connect_event<
        F: Fn(&Self, SocketClientEvent, &SocketConnectable, Option<&IOStream>) + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_enable_proxy_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_family_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_local_address_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_protocol_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_proxy_resolver_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_timeout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_tls_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_tls_validation_flags_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SocketClient>> SocketClientExt for O {
    fn add_application_proxy(&self, protocol: &str) {
        unsafe {
            gio_sys::g_socket_client_add_application_proxy(
                self.as_ref().to_glib_none().0,
                protocol.to_glib_none().0,
            );
        }
    }

    fn connect<P: IsA<SocketConnectable>, Q: IsA<Cancellable>>(
        &self,
        connectable: &P,
        cancellable: Option<&Q>,
    ) -> Result<SocketConnection, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_socket_client_connect(
                self.as_ref().to_glib_none().0,
                connectable.as_ref().to_glib_none().0,
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

    fn connect_async<
        P: IsA<SocketConnectable>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<SocketConnection, Error>) + Send + 'static,
    >(
        &self,
        connectable: &P,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let user_data: Box<R> = Box::new(callback);
        unsafe extern "C" fn connect_async_trampoline<
            R: FnOnce(Result<SocketConnection, Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                gio_sys::g_socket_client_connect_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<R> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = connect_async_trampoline::<R>;
        unsafe {
            gio_sys::g_socket_client_connect_async(
                self.as_ref().to_glib_none().0,
                connectable.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(feature = "futures")]
    fn connect_async_future<P: IsA<SocketConnectable> + Clone + 'static>(
        &self,
        connectable: &P,
    ) -> Box_<dyn future::Future<Output = Result<SocketConnection, Error>> + std::marker::Unpin>
    {
        use fragile::Fragile;
        use GioFuture;

        let connectable = connectable.clone();
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            obj.connect_async(&connectable, Some(&cancellable), move |res| {
                let _ = send.into_inner().send(res);
            });

            cancellable
        })
    }

    fn connect_to_host<P: IsA<Cancellable>>(
        &self,
        host_and_port: &str,
        default_port: u16,
        cancellable: Option<&P>,
    ) -> Result<SocketConnection, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_socket_client_connect_to_host(
                self.as_ref().to_glib_none().0,
                host_and_port.to_glib_none().0,
                default_port,
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

    fn connect_to_host_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static,
    >(
        &self,
        host_and_port: &str,
        default_port: u16,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn connect_to_host_async_trampoline<
            Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_socket_client_connect_to_host_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = connect_to_host_async_trampoline::<Q>;
        unsafe {
            gio_sys::g_socket_client_connect_to_host_async(
                self.as_ref().to_glib_none().0,
                host_and_port.to_glib_none().0,
                default_port,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(feature = "futures")]
    fn connect_to_host_async_future(
        &self,
        host_and_port: &str,
        default_port: u16,
    ) -> Box_<dyn future::Future<Output = Result<SocketConnection, Error>> + std::marker::Unpin>
    {
        use fragile::Fragile;
        use GioFuture;

        let host_and_port = String::from(host_and_port);
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            obj.connect_to_host_async(
                &host_and_port,
                default_port,
                Some(&cancellable),
                move |res| {
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }

    fn connect_to_service<P: IsA<Cancellable>>(
        &self,
        domain: &str,
        service: &str,
        cancellable: Option<&P>,
    ) -> Result<SocketConnection, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_socket_client_connect_to_service(
                self.as_ref().to_glib_none().0,
                domain.to_glib_none().0,
                service.to_glib_none().0,
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

    fn connect_to_service_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static,
    >(
        &self,
        domain: &str,
        service: &str,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn connect_to_service_async_trampoline<
            Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_socket_client_connect_to_service_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = connect_to_service_async_trampoline::<Q>;
        unsafe {
            gio_sys::g_socket_client_connect_to_service_async(
                self.as_ref().to_glib_none().0,
                domain.to_glib_none().0,
                service.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(feature = "futures")]
    fn connect_to_service_async_future(
        &self,
        domain: &str,
        service: &str,
    ) -> Box_<dyn future::Future<Output = Result<SocketConnection, Error>> + std::marker::Unpin>
    {
        use fragile::Fragile;
        use GioFuture;

        let domain = String::from(domain);
        let service = String::from(service);
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            obj.connect_to_service_async(&domain, &service, Some(&cancellable), move |res| {
                let _ = send.into_inner().send(res);
            });

            cancellable
        })
    }

    fn connect_to_uri<P: IsA<Cancellable>>(
        &self,
        uri: &str,
        default_port: u16,
        cancellable: Option<&P>,
    ) -> Result<SocketConnection, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_socket_client_connect_to_uri(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                default_port,
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

    fn connect_to_uri_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static,
    >(
        &self,
        uri: &str,
        default_port: u16,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn connect_to_uri_async_trampoline<
            Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_socket_client_connect_to_uri_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = connect_to_uri_async_trampoline::<Q>;
        unsafe {
            gio_sys::g_socket_client_connect_to_uri_async(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                default_port,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(feature = "futures")]
    fn connect_to_uri_async_future(
        &self,
        uri: &str,
        default_port: u16,
    ) -> Box_<dyn future::Future<Output = Result<SocketConnection, Error>> + std::marker::Unpin>
    {
        use fragile::Fragile;
        use GioFuture;

        let uri = String::from(uri);
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            obj.connect_to_uri_async(&uri, default_port, Some(&cancellable), move |res| {
                let _ = send.into_inner().send(res);
            });

            cancellable
        })
    }

    fn get_enable_proxy(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_socket_client_get_enable_proxy(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_family(&self) -> SocketFamily {
        unsafe {
            from_glib(gio_sys::g_socket_client_get_family(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_local_address(&self) -> Option<SocketAddress> {
        unsafe {
            from_glib_none(gio_sys::g_socket_client_get_local_address(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_protocol(&self) -> SocketProtocol {
        unsafe {
            from_glib(gio_sys::g_socket_client_get_protocol(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_proxy_resolver(&self) -> Option<ProxyResolver> {
        unsafe {
            from_glib_none(gio_sys::g_socket_client_get_proxy_resolver(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_socket_type(&self) -> SocketType {
        unsafe {
            from_glib(gio_sys::g_socket_client_get_socket_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_timeout(&self) -> u32 {
        unsafe { gio_sys::g_socket_client_get_timeout(self.as_ref().to_glib_none().0) }
    }

    fn get_tls(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_socket_client_get_tls(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_tls_validation_flags(&self) -> TlsCertificateFlags {
        unsafe {
            from_glib(gio_sys::g_socket_client_get_tls_validation_flags(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_enable_proxy(&self, enable: bool) {
        unsafe {
            gio_sys::g_socket_client_set_enable_proxy(
                self.as_ref().to_glib_none().0,
                enable.to_glib(),
            );
        }
    }

    fn set_family(&self, family: SocketFamily) {
        unsafe {
            gio_sys::g_socket_client_set_family(self.as_ref().to_glib_none().0, family.to_glib());
        }
    }

    fn set_local_address<P: IsA<SocketAddress>>(&self, address: Option<&P>) {
        unsafe {
            gio_sys::g_socket_client_set_local_address(
                self.as_ref().to_glib_none().0,
                address.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_protocol(&self, protocol: SocketProtocol) {
        unsafe {
            gio_sys::g_socket_client_set_protocol(
                self.as_ref().to_glib_none().0,
                protocol.to_glib(),
            );
        }
    }

    fn set_proxy_resolver<P: IsA<ProxyResolver>>(&self, proxy_resolver: Option<&P>) {
        unsafe {
            gio_sys::g_socket_client_set_proxy_resolver(
                self.as_ref().to_glib_none().0,
                proxy_resolver.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_socket_type(&self, type_: SocketType) {
        unsafe {
            gio_sys::g_socket_client_set_socket_type(
                self.as_ref().to_glib_none().0,
                type_.to_glib(),
            );
        }
    }

    fn set_timeout(&self, timeout: u32) {
        unsafe {
            gio_sys::g_socket_client_set_timeout(self.as_ref().to_glib_none().0, timeout);
        }
    }

    fn set_tls(&self, tls: bool) {
        unsafe {
            gio_sys::g_socket_client_set_tls(self.as_ref().to_glib_none().0, tls.to_glib());
        }
    }

    fn set_tls_validation_flags(&self, flags: TlsCertificateFlags) {
        unsafe {
            gio_sys::g_socket_client_set_tls_validation_flags(
                self.as_ref().to_glib_none().0,
                flags.to_glib(),
            );
        }
    }

    fn get_property_type(&self) -> SocketType {
        unsafe {
            let mut value = Value::from_type(<SocketType as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"type\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `type` getter")
                .unwrap()
        }
    }

    fn set_property_type(&self, type_: SocketType) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"type\0".as_ptr() as *const _,
                Value::from(&type_).to_glib_none().0,
            );
        }
    }

    fn connect_event<
        F: Fn(&Self, SocketClientEvent, &SocketConnectable, Option<&IOStream>) + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn event_trampoline<
            P,
            F: Fn(&P, SocketClientEvent, &SocketConnectable, Option<&IOStream>) + 'static,
        >(
            this: *mut gio_sys::GSocketClient,
            event: gio_sys::GSocketClientEvent,
            connectable: *mut gio_sys::GSocketConnectable,
            connection: *mut gio_sys::GIOStream,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SocketClient>,
        {
            let f: &F = &*(f as *const F);
            f(
                &SocketClient::from_glib_borrow(this).unsafe_cast(),
                from_glib(event),
                &from_glib_borrow(connectable),
                Option::<IOStream>::from_glib_borrow(connection).as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"event\0".as_ptr() as *const _,
                Some(transmute(event_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_enable_proxy_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_enable_proxy_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GSocketClient,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SocketClient>,
        {
            let f: &F = &*(f as *const F);
            f(&SocketClient::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::enable-proxy\0".as_ptr() as *const _,
                Some(transmute(
                    notify_enable_proxy_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_family_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_family_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GSocketClient,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SocketClient>,
        {
            let f: &F = &*(f as *const F);
            f(&SocketClient::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::family\0".as_ptr() as *const _,
                Some(transmute(notify_family_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_local_address_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_local_address_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GSocketClient,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SocketClient>,
        {
            let f: &F = &*(f as *const F);
            f(&SocketClient::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::local-address\0".as_ptr() as *const _,
                Some(transmute(
                    notify_local_address_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_protocol_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_protocol_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GSocketClient,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SocketClient>,
        {
            let f: &F = &*(f as *const F);
            f(&SocketClient::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::protocol\0".as_ptr() as *const _,
                Some(transmute(notify_protocol_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_proxy_resolver_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_proxy_resolver_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GSocketClient,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SocketClient>,
        {
            let f: &F = &*(f as *const F);
            f(&SocketClient::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::proxy-resolver\0".as_ptr() as *const _,
                Some(transmute(
                    notify_proxy_resolver_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_timeout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_timeout_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GSocketClient,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SocketClient>,
        {
            let f: &F = &*(f as *const F);
            f(&SocketClient::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::timeout\0".as_ptr() as *const _,
                Some(transmute(notify_timeout_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_tls_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tls_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GSocketClient,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SocketClient>,
        {
            let f: &F = &*(f as *const F);
            f(&SocketClient::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::tls\0".as_ptr() as *const _,
                Some(transmute(notify_tls_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_tls_validation_flags_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_tls_validation_flags_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GSocketClient,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SocketClient>,
        {
            let f: &F = &*(f as *const F);
            f(&SocketClient::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::tls-validation-flags\0".as_ptr() as *const _,
                Some(transmute(
                    notify_tls_validation_flags_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_type_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GSocketClient,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SocketClient>,
        {
            let f: &F = &*(f as *const F);
            f(&SocketClient::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::type\0".as_ptr() as *const _,
                Some(transmute(notify_type_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SocketClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SocketClient")
    }
}

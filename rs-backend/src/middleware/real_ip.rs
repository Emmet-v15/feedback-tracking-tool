use axum::{
    extract::{ConnectInfo, FromRequestParts},
    http::{HeaderMap, request::Parts},
};
use std::net::{IpAddr, SocketAddr};

pub struct RealIp(pub IpAddr);

impl<S> FromRequestParts<S> for RealIp
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Try to get IP from headers
        if let Some(real_ip) = get_ip_from_headers(&parts.headers) {
            return Ok(RealIp(real_ip));
        }

        // Fall back to the socket address
        if let Some(ConnectInfo(addr)) = parts.extensions.get::<ConnectInfo<SocketAddr>>() {
            return Ok(RealIp(addr.ip()));
        }

        // Last resort: unknown IP
        Ok(RealIp(std::net::IpAddr::V4(std::net::Ipv4Addr::new(
            0, 0, 0, 0,
        ))))
    }
}

pub fn get_ip_from_headers(headers: &HeaderMap) -> Option<IpAddr> {
    // Try X-Real-IP header
    if let Some(real_ip) = headers.get("X-Real-IP") {
        if let Ok(ip_str) = real_ip.to_str() {
            if let Ok(ip) = ip_str.parse::<IpAddr>() {
                return Some(ip);
            }
        }
    }

    // Try X-Forwarded-For header (first IP in the list)
    if let Some(forwarded) = headers.get("X-Forwarded-For") {
        if let Ok(ip_list) = forwarded.to_str() {
            if let Some(first_ip) = ip_list.split(',').next() {
                if let Ok(ip) = first_ip.trim().parse::<IpAddr>() {
                    return Some(ip);
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderMap;
    use axum::http::header::HeaderValue;

    #[test]
    fn test_get_ip_from_x_real_ip() {
        let mut headers = HeaderMap::new();
        headers.insert("X-Real-IP", HeaderValue::from_static("192.168.0.1"));
        assert_eq!(get_ip_from_headers(&headers), Some("192.168.0.1".parse().unwrap()));
    }

    #[test]
    fn test_get_ip_from_x_forwarded_for() {
        let mut headers = HeaderMap::new();
        headers.insert("X-Forwarded-For", HeaderValue::from_static("10.0.0.1, 10.0.0.2"));
        assert_eq!(get_ip_from_headers(&headers), Some("10.0.0.1".parse().unwrap()));
    }

    #[test]
    fn test_get_ip_from_headers_none() {
        let headers = HeaderMap::new();
        assert_eq!(get_ip_from_headers(&headers), None);
    }
}

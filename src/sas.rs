fn generate_sas(hub: &str, device_id: &str, key: &str, expiry_timestamp: i64) -> String {
    let resource_uri = format!("{}/devices/{}", hub, device_id);

    const FRAGMENT: &percent_encoding::AsciiSet = &percent_encoding::CONTROLS.add(b'/');

    let resource_uri = percent_encoding::utf8_percent_encode(&resource_uri, FRAGMENT);
    let to_sign = format!("{}\n{}", &resource_uri, expiry_timestamp);

    let key = base64::decode(&key).unwrap();
    let mut mac = Hmac::<Sha256>::new_varkey(&key).unwrap();
    mac.input(to_sign.as_bytes());
    let mac_result = mac.result().code();
    let signature = base64::encode(mac_result.as_ref());

    let pairs = &vec![("sig", signature)];
    let token = serde_urlencoded::to_string(pairs).unwrap();

    let sas = format!(
        "SharedAccessSignature sr={}&{}&se={}",
        resource_uri, token, expiry_timestamp
    );

    sas
}

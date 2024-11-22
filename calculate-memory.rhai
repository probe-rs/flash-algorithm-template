variable::set(
    "ram-end-address",
    "0x" + (
        parse_int(variable::get("ram-start-address").sub_string(2), 16)
      + parse_int(variable::get("ram-size").sub_string(2), 16)
    ).to_hex()
);
variable::set(
    "flash-end-address",
    "0x" + (
        parse_int(variable::get("flash-start-address").sub_string(2), 16)
      + parse_int(variable::get("flash-size").sub_string(2), 16)
    ).to_hex()
);
variable::set(
  "link-address",
  "0x" + (
      parse_int(variable::get("ram-start-address").sub_string(2), 16)
    + 0x20
  ).to_hex()
);

let target = variable::get("target-arch");

variable::set("architecture", switch target {
    "thumbv6m-none-eabi" => "armv6m",
    "thumbv7m-none-eabi" => "armv7m",
    "thumbv7em-none-eabi" => "armv7em",
    "thumbv7em-none-eabihf" => "armv7em",
    "thumbv8m.base-none-eabi" => "armv8",
    "thumbv8m.main-none-eabi" => "armv8",
    "thumbv8m.main-none-eabihf" => "armv8",
    _ => "armv6m",
})
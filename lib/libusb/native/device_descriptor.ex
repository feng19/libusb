defmodule LibUSB.Native.DeviceDescriptor do
  @moduledoc false

  defstruct [
    :vendor_id,
    :product_id,
    :class_code,
    :sub_class_code,
    :usb_version,
    :device_version,
    :protocol_code,
    :max_packet_size,
    :manufacturer_index,
    :product_index,
    :serial_number_index,
    :num_configurations
  ]
end

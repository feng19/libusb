defmodule LibUSB.Native.DeviceDescriptor do
  @moduledoc false

  @type version :: {non_neg_integer, non_neg_integer, non_neg_integer}
  @type t :: %__MODULE__{
          vendor_id: integer,
          product_id: integer,
          class_code: integer,
          sub_class_code: integer,
          usb_version: version,
          device_version: version,
          protocol_code: integer,
          max_packet_size: integer,
          manufacturer_index: integer,
          product_index: integer,
          serial_number_index: integer,
          num_configurations: integer
        }

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

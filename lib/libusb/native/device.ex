defmodule LibUSB.Native.Device do
  @moduledoc false
  @type t :: %__MODULE__{
          bus_number: integer,
          address: integer,
          port_numbers: integer,
          device_descriptor: LibUSB.Native.DeviceDescriptor.t()
        }

  defstruct [:bus_number, :address, :port_numbers, :device_descriptor]
end

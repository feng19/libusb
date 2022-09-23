defmodule LibUSB.Native.Device do
  @moduledoc false
  defstruct [:bus_number, :address, :port_numbers, :device_descriptor]
end

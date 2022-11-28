defmodule LibUSB do
  @moduledoc false
  alias LibUSB.Native

  defdelegate info, to: Native
  defdelegate list_devices, to: Native
  defdelegate open(vendor_id, product_id), to: Native
  defdelegate write_bulk(device_handle, raw, timeout \\ 2), to: Native
  defdelegate read_bulk(device_handle, timeout \\ 2), to: Native
end

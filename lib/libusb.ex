defmodule LibUSB do
  @moduledoc false
  alias LibUSB.Native

  @type vendor_id :: integer
  @type product_id :: integer
  @type device :: {vendor_id, product_id}
  @type device_handle :: reference

  @spec info :: %{
          has_capability: boolean,
          has_hid_access: boolean,
          has_hotplug: boolean,
          supports_detach_kernel_driver: boolean
        }
  defdelegate info, to: Native

  @spec list_devices :: [device]
  def list_devices do
    for %{device_descriptor: %{vendor_id: vendor_id, product_id: product_id}} <-
          list_devices_detail() do
      {vendor_id, product_id}
    end
  end

  @spec list_devices_detail :: [LibUSB.Native.Device.t()]
  defdelegate list_devices_detail, to: Native, as: :list_devices

  @spec write(vendor_id, product_id, raw :: binary, timeout) ::
          {:ok, integer()} | {:error, error :: String.t()}
  defdelegate write(vendor_id, product_id, raw, timeout \\ 2), to: Native
end

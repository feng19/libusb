defmodule LibUSB.Native do
  @moduledoc false
  use Rustler, otp_app: :libusb, crate: "libusb_nif"

  def info, do: error()
  def list_devices, do: error()
  def open(_vendor_id, _product_id), do: error()
  def write_bulk(_device_handle, _raw, _timeout \\ 2), do: error()
  def read_bulk(_device_handle, _timeout \\ 2), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end

defmodule LibUSB.Native do
  @moduledoc false
  use Rustler, otp_app: :libusb, crate: "libusb_nif"

  def info, do: error()
  def list_devices, do: error()
  def write(_vendor_id, _product_id, _raw, _timeout \\ 2), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end

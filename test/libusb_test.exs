defmodule LibusbTest do
  use ExUnit.Case
  doctest Libusb

  test "greets the world" do
    assert Libusb.hello() == :world
  end
end

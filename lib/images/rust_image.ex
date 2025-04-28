defmodule Images.RustImage do
  use Rustler, otp_app: :images, crate: "images_rustimage"

  def add(_a, _b) do
    :erlang.nif_error(:nif_not_loaded)
  end

  def jpg(_input, _output, _quality) do
    :erlang.nif_error(:nif_not_loaded)
  end
end

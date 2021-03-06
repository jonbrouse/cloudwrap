# brew install https://raw.githubusercontent.com/scirner22/cloudwrap/master/ci/homebrew.rb

require  'formula'
class Homebrew < Formula
  _user = 'scirner22'
  _repo_name = 'cloudwrap'
  _bin_name = 'cloudwrap'
  homepage "https://github.com/#{_user}/#{_repo_name}"
  _ver = '0.2.2'
  version _ver

  url "https://data.blackfynn.io.s3.amazonaws.com/public-downloads/#{_repo_name}/#{_bin_name}_#{_ver}_x86_64-apple-darwin.tar.gz"
  sha256 '07f31d0479e5d541f5aa44cc5205a3e5373f429ae58a6668109a9e4aabb0f333'

  def install
    bin.install _bin_name
  end
end

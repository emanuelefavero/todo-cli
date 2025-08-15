# How to Release Your Own Version of `todo-cli`

> Note: These steps are also useful for creating a new version of the app (e.g. v2.0.0)

## Create a GitHub Release and Upload the Binary

1. Go to your GitHub repo page (e.g. `https://github.com/emanuelefavero/todo-cli`)
2. Click on "Releases" -> "Draft a new release"
3. Set the tag version (e.g. `v1.0.0`)
4. Add a title (e.g. `todo v1.0.0`) and description
5. Upload the `todo` binary from the `target/release` directory
6. Publish the release

> Do the same for future versions

## Get the Download Link and SHA256

1. After publishing the release, you will see the download link for the binary.
2. To get the SHA256 checksum, run the following commands:

   ```bash
   curl -L -o todo https://github.com/emanuelefavero/todo-cli/releases/download/v1.0.0/todo
   shasum -a 256 todo
   ```

> Do the same for future versions

## Create a Homebrew Tap and Formula

1. Create a new GitHub repo named `homebrew-tap` (e.g., `emanuelefavero/homebrew-tap`).
2. In that repo, create a file: `Formula/todo.rb`.
3. Use this template (replace the URL and SHA256):

> Tip: When creating a new version, update the `url` and `sha256` fields accordingly.

```ruby
class Todo < Formula
  desc "Your todo CLI description"
  homepage "https://github.com/emanuelefavero/todo-cli"
  url "https://github.com/emanuelefavero/todo-cli/releases/download/v1.0.0/todo"
  # Example: 34ded66014650eddc829be4c7fe3d53010fadg377b95f515d7b4dc4cd4f4dcc4
  sha256 "YOUR_SHA256_CHECKSUM_HERE"
  version "1.0.0"

  def install
    bin.install "todo"
  end

  test do
    system "#{bin}/todo", "--help"
  end
end
```

> Commit and push the file to your `homebrew-tap` repo.

## Now you can install your formula using Homebrew

```bash
brew tap emanuelefavero/tap
brew install todo
```

[Go Back to README](README.md)

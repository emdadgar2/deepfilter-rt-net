# DeepFilterRt.Interop

This project wraps the native `deepfilter_rt` DLL created from the Rust library. It targets .NET 10 and exposes a simple C# API for P/Invoke interop.

## Expected DLL exports

The native DLL must expose these functions:

```c
IntPtr dfn_create(string modelDirectory);
ulong dfn_process(IntPtr handle, float* input, float* output, ulong length);
void dfn_destroy(IntPtr handle);
```

## Example

```csharp
using DeepFilterRt.Interop;

using var net = new DeepFilterNet(@"C:\models\dfn3");

float[] input = new float[480];
float[] output = new float[480];

int produced = net.Process(input, output);
Console.WriteLine($"Produced {produced} samples.");
```

## Notes

- The native DLL should be copied beside the app output or placed under a `native/` or `runtimes/*/native/` directory.
- The exact output length may be shorter than the input length because the Rust stream pipeline buffers data internally.

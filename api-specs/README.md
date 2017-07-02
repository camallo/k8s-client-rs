# Origin

Specs are downloaded from
```
https://raw.githubusercontent.com/kubernetes/kubernetes/release-<x.x>/api/openapi-spec/swagger.json
```

# Code generation

In order to trigger code generation for some specific k8s version, first prepare the environment:

```
cp -a api-specs/skeleton api-specs/vx_y
wget -O api-specs/vx_y/ https://raw.githubusercontent.com/kubernetes/kubernetes/release-<x.x>/api/openapi-spec/swagger.json
editor api-specs/vx_y/config.yaml
```

Then run the code-generator:
```
cargo run -p k8s-rsgen -- -c api-specs/vx_y/swagger.json
```

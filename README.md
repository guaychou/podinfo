# Pod Info 

### For educational purpose and learn golden metric in service mesh

### Endpoint
- /
- /notfound
- /internalservererror

### Response 

- Ok
```json
{
  "code": 200,
  "hostname": "system-76",
  "message": "Hello world !"
}
```


- Not Found

```json
{
  "code": 404,
  "hostname": "system-76",
  "message": "Uhh ohh, not found !"
}
```

- Service Unavailable

```json
{
  "code": 503,
  "hostname": "system-76",
  "message": "Uhh ohh, service is unavailable"
}
```
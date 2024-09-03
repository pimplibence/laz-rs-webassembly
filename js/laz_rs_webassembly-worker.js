onmessage = async (event) => {
    const libPath = event.data.libPath;
    const wasmPath = event.data.wasmPath;

    const data = event.data.data;
    const module = await import(libPath);
    const instance = await module.default(wasmPath);
    const result = module.read_sync(data);

    const response = {
        header: {
            major: result.header.major,
            minor: result.header.minor,
            offset_to_points: result.header.offset_to_points,
            num_vlrs: result.header.num_vlrs,
            point_format_id: result.header.point_format_id,
            point_size: result.header.point_size,
            num_points: result.header.num_points,
            header_size: result.header.header_size,
            scale_x: result.header.scale_x,
            scale_y: result.header.scale_y,
            scale_z: result.header.scale_z,
            offset_x: result.header.offset_x,
            offset_y: result.header.offset_y,
            offset_z: result.header.offset_z,
            max_x: result.header.max_x,
            max_y: result.header.max_y,
            max_z: result.header.max_z,
            min_x: result.header.min_x,
            min_y: result.header.min_y,
            min_z: result.header.min_z,
        },
        coordinates: new Float64Array(instance.memory.buffer, result.coordinates_pointer, result.coordinates_length),
        intensities: new Uint16Array(instance.memory.buffer, result.intensity_pointer, result.intensity_length),
        classifications: new Uint8Array(instance.memory.buffer, result.classification_pointer, result.classification_length)
    };

    postMessage(response)
}

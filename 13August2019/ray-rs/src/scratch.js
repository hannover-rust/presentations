
var js_render_1 = () => {
    let img = ctx.getImageData(0, 0, width, height);
    let raw_data = img.data;
    let w = 600
    let ambient_light = 2;
    let lights = [
        8, [2, 2, 0]
    ]
    let math = Math
    let sqrt = math.sqrt
    let max = math.max
    let h = width / 2.0
    let out_idx = 0
    let spheres = [
        w, [0, -w, 0], 9, 9, 0, w, 2,  // Yellow sphere
        1, [0, 0, 3], 9, 0, 0, w, 3,  // Red sphere
        1, [-2, 1, 4], 0, 9, 0, 9, 4,  // Green sphere
        1, [2, 1, 4], 0, 0, 9, w, 5   // Blue sphere
    ];
    let dot = (A, B) => { return A[0] * B[0] + A[1] * B[1] + A[2] * B[2]; }
    let A_minus_Bk = (A, B, k) => { return [A[0] - B[0] * k, A[1] - B[1] * k, A[2] - B[2] * k]; }
    let closest_intersection = (B, D, t_min, t_max) => {
        let t = w;  // Min distance found.
        // Quadratic equation coefficients are K1, K2, K3. K1 is constant for the ray.
        let a = 2 * dot(D, D);  // 2*K1

        // For each sphere.
        // Get the radius and test for end of array at the same time; 
        // spheres[n] == undefined ends the loop.
        // q points to the 2nd element of the sphere because of q++; +6 skips to next
        // sphere.
        var q = 0;
        var v = 0;
        var d = 0;
        var r = spheres[0]
        for (; r = spheres[q++]; q += 6) {
            var j = A_minus_Bk(B, spheres[q], 1)
            let b = -2 * dot(j, D);  // -K2; also j = origin - center

            // Compute sqrt(Discriminant) = sqrt(K2*K2 - 4*K1*K3), go ahead if there are
            // solutions.
            if (d = sqrt(b * b - 2 * a * (dot(j, j) - r * r))) {
                // Compute the two solutions.
                for (var e = 2; e--; d = -d) {
                    let f = (b - d) / a;  // f = (-K2 - d) / 2*K1
                    if (t_min < f && f < t_max && f < t) {
                        v = q;
                        t = f;
                    }
                }
            }
        }
        // Return index of closest sphere in range; t is global
        return v;
    }
    var trace_ray = (B, D, t_min, t_max, depth) => {
        // Find nearest hit; if no hit, return black.
        let s = 0
        if (!(s = closest_intersection(B, D, t_min, t_max))) {
            return 0;
        }

        // Compute "normal" at intersection: N = X - spheres[s]
        let N = A_minus_Bk(
            X = A_minus_Bk(B, D, -t),  // intersection: X = B + D*t = B - D*(-t)
            spheres[s], 1);

        // Instead of normalizing N, we divide by its length when appropriate. Most of
        // the time N appears twice, so we precompute its squared length.
        let n = dot(N, N);

        // Start with ambient light only
        let i = ambient_light;

        // For each light
        for (var l = 0; u = lights[l++];) { // Get intensity and check for end of array

            // Compute vector from intersection to light (L = lights[l++] - X) and
            // k = <N,L> (reused below)
            let k = dot(N, L = A_minus_Bk(lights[l++], X, 1));

            // Add to lighting
            i += u *
                // If the pont isn't in shadow
                // [t_min, t_max]  =  [epsilon,  1] - epsilon avoids self-shadow, 1 
                // doesn't look farther than the light itself.
                !closest_intersection(X, L, 1 / w, 1) * (
                    // Diffuse lighting, only if it's facing the point 
                    // <N,L> / (|N|*|L|) = cos(alpha)
                    // Also, |N|*|L| = sqrt(<N,N>)*sqrt(<L,L>) = sqrt(<N,N>*<L,L>)
                    max(0, k / sqrt(dot(L, L) * n))

                    // Specular highlights
                    //
                    // specular = (<R,V>   / (|R|*|V|))   ^ exponent
                    //          = (<-R,-V> / (|-R|*|-V|)) ^ exponent
                    //          = (<-R,D>  / (|-R|*|D|))  ^ exponent
                    //
                    // R = 2*N*<N,L> - L
                    // M = -R = -2*N*<N,L> + L = L + N*(-2*<N,L>)
                    //
                    // If the resultant intensity is negative, treat it as 0 (ignore it).
                    + max(0, math.pow(dot(M = A_minus_Bk(L, N, 2 * k / n), D)
                        / sqrt(dot(M, M) * dot(D, D)), spheres[s + 4]))
                );
        }


        // Compute the color channel multiplied by the light intensity. 2.8 maps
        // the color range from [0, 9] to [0, 255] and the intensity from [0, 10]
        // to [0, 1],  because 2.8 ~ (255/9)/10
        // 
        // spheres[s] = sphere center, so spheres[s+c] = color channel
        // (c = [1..3] because ++c below)
        var local_color = spheres[s + c] * i * 2.8;

        // If the recursion limit hasn't been hit yet, trace reflection rays.
        // N = normal (non-normalized - two divs by |N| = div by <N,N>
        // D = -view
        // R = 2*N*<N,V>/<N,N> - V = 2*N*<N,-D>/<N,N> + D = D - N*(2*<N,D>/<N,N>)
        var ref = spheres[s + 5] / 9;
        return depth-- ? trace_ray(X,
            A_minus_Bk(D, N, 2 * dot(N, D) / n),  // R
            1 / w, w, depth) * ref
            + local_color * (1 - ref)
            : local_color;
    }

    // For each y; also compute h=w/2 without paying an extra ";"
    for (var y = h = w / 2; y-- > -h;) {

        // For each x
        for (var x = -h; x++ < h;) {

            // One pass per color channel (!). This way we don't have to deal with
            // "colors".
            for (var c = 0; ++c < 4;) {
                // Camera is at (0, 1, 0)
                //
                // Ray direction is (x*vw/cw, y*vh/ch, 1) where vw = viewport width, 
                // cw = canvas width (vh and ch are the same for height). vw is fixed
                // at 1 so (x/w, y/w, 1)
                //
                // [t_min, t_max] = [1, w], 1 starts at the projection plane, w is +inf
                //
                // 2 is a good recursion depth to appreciate the reflections without
                // slowing things down too much
                //
                raw_data[out_idx++] = trace_ray([0, 1, 0], [x / w, y / w, 1], 1, w, 2);
            }
            raw_data[out_idx++] = 255; // Opaque alpha
        }
    }
    return 
}
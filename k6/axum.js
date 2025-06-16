// import necessary module
import { check } from 'k6';
import http from 'k6/http';

export const options = {
    // smoke test conditions
    thresholds: {
        http_req_failed: [{threshold: 'rate<0.00001', abortOnFail: true}], // http errors should be less than 0.001%
        http_req_duration: ['p(99)<5'], // 99% of requests should be below 5000µs, 5ms or 0.005s
    },
};

export default function () {
    const url = 'http://localhost:8080/v1/transform';
    const payload = JSON.stringify({
        transform: 'lowercase',
        html: '<div><p>First paragraph element</p><span>Not a paragraph</span><p>This is the <strong>Second</strong> listed <em>Paragraph</em> element</p></div>',
    });

    const params = {
        headers: {
            'Content-Type': 'application/json',
        },
    };

    // send a post request and save response as a variable
    const res = http.post(url, payload, params);

    check(res, {
        'response was 200': (res) => res.status === 200,
    })
}
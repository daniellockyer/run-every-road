#!/usr/bin/env node

const filter = require('through2-filter').obj;
const geojsonStream = require('geojson-stream');
const got = require('got');
const Readable = require('stream').Readable;
const through2 = require('through2').obj;
const util = require('util');

if (process.argv.length !== 3) {
    throw new Error('Please supply the Strava token as a parameter');
}

const STRAVA_TOKEN = process.argv[2];

class Source {
    constructor() {
        Readable.call(this, {
            objectMode: true
        });
        this.page = 1;
    }

    async _read() {
        this.pause();

        const res = await got(`https://www.strava.com/api/v3/athlete/activities?per_page=100&page=${this.page}&after=1601247600`, {
            headers: {
                'Authorization': `Bearer ${STRAVA_TOKEN}`,
                'Content-Type': 'application/json'
            },
        }).json();

        this.page++;
        this.resume();

        if (res.length) this.push(res);
        if (res.length < 100) {
            this.push(null);
            this.emit('end');
        }
    }
}

util.inherits(Source, Readable);

new Source()
    .pipe(through2(function (chunk, enc, callback) {
        chunk.forEach(c => {
            this.push(c.id);
        });
        callback();
    }))
    .pipe(through2(async function (id, enc, callback) {
        const res = await got(`https://www.strava.com/api/v3/activities/${id}/streams/latlng`, {
            headers: {
                'Authorization': `Bearer ${STRAVA_TOKEN}`,
                'Content-Type': 'application/json'
            },
        }).json();

        callback(null, res);
    }))
    .pipe(filter(Array.isArray))
    .pipe(through2(async function (chunk, enc, callback) {
        if (!chunk || !chunk.filter) return;

        callback(null, chunk.filter(e => e.type === 'latlng').map(e => ({
            type: 'Feature',
            geometry: {
                type: 'LineString',
                coordinates: e.data.map(coord => coord.slice().reverse())
            },
            properties: {}
        }))[0]);
    }))
    .pipe(geojsonStream.stringify())
    .pipe(process.stdout);

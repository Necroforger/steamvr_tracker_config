import { fs, tauri, shell, invoke, path } from "@tauri-apps/api"



/**
 * 
 * @param {string} executable The executable path for the steamvr lighthouse settings
 * @param {string} text  json encoded data to upload to the lighthouse
 * @returns {Promise<string>}
 */
async function upload_lighthouse_config(executable, text) {
    let dirname = await make_tempdir()
    let fname = dirname + "/config.json"
    await fs.writeTextFile(fname, text)

    console.log("Invoking run_command")
    const result = await invoke("run_command", {
        command: executable,
        workdir: dirname,
        arg: "uploadconfig config.json",
    })

    console.log("removing files")

    setTimeout(async () => {
        await fs.removeFile(fname)
        await fs.removeDir(dirname)
    }, 300)

    return result
}

/**
 * Writes a backup  of the given text content into the backups directory
 * @param {string} name 
 * @param {string} content 
 */
async function write_backup(name, content) {
    try {
        await fs.createDir("backups")
    } catch(_) { }
    await fs.writeTextFile(await path.join("backups/", name), content)
}

/**
 * reads a backup by name 
 * @param {string} name 
 */
async function read_backup(name) {
    return await fs.readTextFile(await path.join("backups", name))
}

/**
 * checks if a named backup exists 
 * @param {string} name 
 * @returns {Promise<boolean>}
 */
async function backup_exists(name) {
    return await fs.exists(await path.join("backups", name))
}

async function get_lighthouse_config(executable) {
    let dirname = await make_tempdir()
    let result = await invoke("run_command", {
        command: executable,
        arg:     "downloadconfig",
        workdir: dirname,
    })

    console.log(result)
    console.log("reading downloaded json file")

    let files = await fs.readDir(dirname)
    console.log(files)
    if (files.length == 0) {
        await fs.removeDir(dirname)
        throw "There was no json file downloaded, There may not be any connected steamvr devices"
    }

    
    let file_contents = await fs.readTextFile(files[0].path)
    console.log("removing temporary dir")

    await fs.removeFile(files[0].path)
    await fs.removeDir(dirname)

    return file_contents
}

async function make_tempdir() {
    let now = Date.now()
    let dirname = `temp/${now}`
    await fs.createDir(dirname, {
        recursive: true,
    })
    console.log("Created directory " + dirname)
    return dirname
}

export {
    read_backup,
    backup_exists,
    upload_lighthouse_config,
    write_backup,
    get_lighthouse_config,
    make_tempdir,
}
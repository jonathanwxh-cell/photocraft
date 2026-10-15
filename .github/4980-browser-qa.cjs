const { chromium } = require('playwright');
const fs = require('fs');
const assert = require('node:assert/strict');
(async () => {
  fs.mkdirSync('/tmp/qa-inputs', {recursive:true});
  fs.mkdirSync('qa-evidence', {recursive:true});
  // A tiny valid public-domain generated PNG; no stress-size allocation.
  const png = Buffer.from('iVBORw0KGgoAAAANSUhEUgAAAAgAAAAICAYAAADED76LAAAAEklEQVR4nGP478DwHx9mGBkKAAtej4ETCPjnAAAAAElFTkSuQmCC', 'base64');
  const files = Array.from({length:33}, (_, i) => `/tmp/qa-inputs/qa-${i}.png`);
  files.forEach(f => fs.writeFileSync(f,png));
  const browser = await chromium.launch({headless:true,args:['--no-sandbox','--use-angle=swiftshader','--enable-unsafe-swiftshader']});
  const page = await browser.newPage({viewport:{width:1400,height:1000}});
  const errors=[];
  page.on('pageerror',e=>errors.push(String(e)));
  await page.addInitScript(() => {
    window.qaReads=[];
    const read=Blob.prototype.arrayBuffer;
    Blob.prototype.arrayBuffer=function(...args) {
      if(this instanceof File && this.name.startsWith('qa-')) window.qaReads.push({name:this.name,size:this.size});
      return read.apply(this,args);
    };
  });
  try {
    await page.goto('http://127.0.0.1:8765/?webgl&cpu');
    await page.locator('#photocraft_loading').waitFor({state:'detached',timeout:180000});
    await page.locator('canvas').click({position:{x:700,y:450}});
    await page.screenshot({path:'qa-evidence/00-start.png'});
    async function pick(paths) {
      const chooser=page.waitForEvent('filechooser',{timeout:15000});
      await page.keyboard.press('Control+o');
      await (await chooser).setFiles(paths);
      await page.waitForTimeout(2000);
    }
    async function drop(paths) {
      const cdp=await page.context().newCDPSession(page);
      const data={items:[],files:paths,dragOperationsMask:1};
      await cdp.send('Input.dispatchDragEvent',{type:'dragEnter',x:700,y:450,data});
      await cdp.send('Input.dispatchDragEvent',{type:'drop',x:700,y:450,data});
      await cdp.detach();
      await page.waitForTimeout(2000);
    }
    const reads=()=>page.evaluate(()=>window.qaReads.length);
    await pick([files[0]]);
    assert.equal(await reads(),1,'normal picker read');
    await page.screenshot({path:'qa-evidence/01-picker.png'});
    await drop([files[1]]);
    assert.equal(await reads(),2,'normal drop read');
    await page.screenshot({path:'qa-evidence/02-drop.png'});
    await pick(files);
    assert.equal(await reads(),2,'over-count picker rejected before any reads');
    await page.screenshot({path:'qa-evidence/03-picker-rejected.png'});
    await drop(files);
    assert.equal(await reads(),2,'over-count drop rejected before any reads');
    await page.screenshot({path:'qa-evidence/04-drop-rejected.png'});
    await pick([files[2],files[3]]);
    assert.equal(await reads(),4,'normal multi-picker still works after rejection');
    await page.screenshot({path:'qa-evidence/05-retry.png'});
    assert.deepEqual(await page.evaluate(()=>window.qaReads.map(r=>r.name)),['qa-0.png','qa-1.png','qa-2.png','qa-3.png'],'normal files read once and in selection order');
    assert.deepEqual(errors,[],'no uncaught browser errors');
    fs.writeFileSync('qa-evidence/browser.json',JSON.stringify({readCalls:await page.evaluate(()=>window.qaReads),errors},null,2));
    console.log('Browser picker/drop acquisition and rejection checks passed. Screenshots require visual review.');
  } catch(e) {
    await page.screenshot({path:'qa-evidence/failure.png'}).catch(()=>{});
    fs.writeFileSync('qa-evidence/failure.json',JSON.stringify({error:String(e),errors},null,2));
    throw e;
  } finally { await browser.close(); }
})().catch(e=>{ console.error(e); process.exitCode=1; });

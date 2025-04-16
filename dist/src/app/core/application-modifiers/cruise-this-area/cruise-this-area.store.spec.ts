import { TestBed } from '@angular/core/testing';
import { CruiseThisAreaStore } from './cruise-this-area.store';


const INPUT_FOR_CAN_RENDER: boolean = /* Add input here */;

const INPUT_FOR_RADIUS: number = /* Add input here */;


describe('CruiseThisAreaStore', () => {
  let store: CruiseThisAreaStore;

  beforeEach(() => {
    store = TestBed.inject(CruiseThisAreaStore);
  });

  describe('methods', () => {
    
    describe('setCanRender', () => {
      it ('should update the CanRender', () => {
        let input = INPUT_FOR_CAN_RENDER;

        store.setCanRender(input);
        expect(store.canRender()).toBe(input);
      });
    });
    
    describe('setRadius', () => {
      it ('should update the Radius', () => {
        let input = INPUT_FOR_RADIUS;

        store.setRadius(input);
        expect(store.radius()).toBe(input);
      });
    });
    
  });
});

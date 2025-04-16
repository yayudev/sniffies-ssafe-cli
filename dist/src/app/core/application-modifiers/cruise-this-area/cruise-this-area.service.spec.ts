import { CruiseThisAreaService } from "./cruise-this-area.service";
import { CruiseThisAreaStore } from "./cruise-this-area.store";


const INPUT_FOR_CAN_RENDER: boolean = /* PUT_YOUR_TEST_INPUT_HERE */ ;

const INPUT_FOR_RADIUS: number = /* PUT_YOUR_TEST_INPUT_HERE */ ;


describe("CruiseThisAreaService", () => {
  let store: CruiseThisAreaStore;
  let service: CruiseThisAreaService;

  beforeEach(() => {
    store = inject(CruiseThisAreaStore);
    service = inject(CruiseThisAreaService);
  });

  
    describe('setCanRender', () => {
      it('should set canRender when called', () => {
        const input = INPUT_FOR_CANRENDER;
        service.setCanRender(input);

        expect(store.canRender()).toBe(input);
      })
    });
  
    describe('setRadius', () => {
      it('should set radius when called', () => {
        const input = INPUT_FOR_RADIUS;
        service.setRadius(input);

        expect(store.radius()).toBe(input);
      })
    });
  
});

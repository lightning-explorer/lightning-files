import { ComponentFixture, TestBed } from '@angular/core/testing';

import { TabsHolderComponent } from './tabs-holder.component';

describe('TabsHolderComponent', () => {
  let component: TabsHolderComponent;
  let fixture: ComponentFixture<TabsHolderComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [TabsHolderComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(TabsHolderComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

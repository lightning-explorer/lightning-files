import { ComponentFixture, TestBed } from '@angular/core/testing';

import { IconifyIconClusterComponent } from './iconify-icon-cluster.component';

describe('IconifyIconClusterComponent', () => {
  let component: IconifyIconClusterComponent;
  let fixture: ComponentFixture<IconifyIconClusterComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [IconifyIconClusterComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(IconifyIconClusterComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
